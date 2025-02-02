use egui_macroquad::egui::Widget;
use egui_macroquad::egui::{Align2, Color32};

use crate::{cpu, egui, Memory};

pub struct App {
	pub paused: bool,
	pub step: bool,
	pub instructions_per_frame: u32,
	/// A debug symbols relates a line from the source code to the corresponding
	/// u16 program counter address
	pub debug_symbols: Vec<u16>,
	pub source_file: Vec<String>,
	pub reset: bool,
	/// Vector of line numbers
	breakpoints: Vec<usize>,
	breakpoints_user_entry: String,
	watchpoints: Vec<u16>,
	watchpoints_user_entry: String,
	ui_scale: f32,
}

impl App {
	pub fn new(debug_symbols: Vec<u16>, source_file: Vec<String>) -> Self {
		Self {
			step: false,
			paused: false,
			instructions_per_frame: 100,
			debug_symbols,
			source_file,
			reset: false,
			breakpoints: vec![],
			breakpoints_user_entry: String::new(),
			watchpoints: vec![],
			watchpoints_user_entry: String::new(),
			ui_scale: 1.,
		}
	}
	pub fn render_ui(&mut self, ctx: &egui::Context, cpu: &cpu::Cpu, mem: &Memory) {
		let current_line_number = self
			.debug_symbols
			.iter()
			.position(|&i| i == cpu.state().program_counter)
			.unwrap();
		egui::Window::new("Debug Controls").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("UI Scale: ");
				if ui
					.add(egui::Slider::new(&mut self.ui_scale, 0.5f32..=3.))
					.drag_released()
				{
					ctx.set_pixels_per_point(self.ui_scale);
				};
			});
			ui.horizontal(|ui| {
				ui.label("Simulation Speed: ");
				ui.add(egui::Slider::new(
					&mut self.instructions_per_frame,
					1u32..=500,
				))
			});
			ui.horizontal(|ui| {
				if ui
					.add(egui::Button::new(if !self.paused {
						"Pause Execution"
					} else {
						"Resume Execution"
					}))
					.clicked()
				{
					self.paused = !self.paused;
				};
				if ui.add(egui::Button::new("Reset")).clicked() {
					self.reset = true
				};
				if self.paused {
					if ui.add(egui::Button::new("Step")).clicked() {
						self.step = true
					};
				}
			});
		});
		if self.paused {
			let cpu_state = cpu.state();
			egui::Window::new("CPU State").show(ctx, |ui| {
				ui.add(egui::Label::new(format!(
					"Prgram Counter: 0x{:0x}",
					cpu_state.program_counter
				)));
				ui.add(egui::Label::new(format!(
					"Stack Pointer: 0x{:0x}",
					cpu_state.stack_pointer
				)));
				ui.add(egui::Label::new(format!(
					"Line Number: {}",
					if let Some(line_number) = self
						.debug_symbols
						.iter()
						.position(|&i| i == cpu_state.program_counter)
					{
						(line_number + 1).to_string()
					} else {
						"xxx".to_string()
					}
				)));
				ui.add(egui::Label::new("Instruction:"));
				ui.label(
					egui::RichText::new(&self.source_file[current_line_number])
						.color(Color32::YELLOW),
				);
				ui.add(egui::Label::new("Registers:"));
				ui.label(
					egui::RichText::new(format!(
						"A: 0x{:02x}, X: 0x{:02x}, Y: 0x{:02x}",
						cpu_state.a, cpu_state.x, cpu_state.y
					))
					.color(Color32::LIGHT_RED),
				)
			});
		}
		egui::Window::new("Source Code")
			.anchor(Align2::RIGHT_TOP, [-10., 10.])
			.show(ctx, |ui| {
				egui::ScrollArea::vertical()
					.max_height(f32::INFINITY)
					.show(ui, |ui| {
						self.source_file
							.iter()
							.enumerate()
							.for_each(|(line_number, line)| {
								ui.horizontal(|ui| {
									if egui::Label::new(
										egui::RichText::new(format!(
											"{}{}\t",
											" ".repeat(3 - (line_number + 1).to_string().len()),
											line_number + 1
										))
										.background_color(if line_number == current_line_number {
											Color32::RED
										} else if self.breakpoints.contains(&(line_number + 1)) {
											Color32::BLUE
										} else {
											Color32::default()
										}),
									)
									.sense(egui::Sense::click())
									.ui(ui)
									.clicked()
									{
										if let Some(index) = self
											.breakpoints
											.iter()
											.position(|&i| i == line_number + 1)
										{
											self.breakpoints.remove(index);
										} else {
											self.breakpoints.push(line_number + 1);
										}
									};
									ui.label(
										egui::RichText::new(line)
											.color(if line.contains(";") {
												Color32::DARK_GREEN
											} else {
												Color32::YELLOW
											})
											.background_color(
												if current_line_number == line_number {
													Color32::RED
												} else {
													Color32::default()
												},
											),
									);
								});
							})
					});
			});
		egui::Window::new("Breakpoints").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Line number:");
				if ui
					.add(
						egui::TextEdit::singleline(&mut self.breakpoints_user_entry)
							.desired_width(40.),
					)
					.lost_focus() || ui.button("Add").clicked()
				{
					if let Ok(line_number) = self.breakpoints_user_entry.parse() {
						if !self.breakpoints.contains(&line_number) {
							self.breakpoints.push(line_number);
						}
					}
					self.breakpoints_user_entry.clear();
				}
			});
			let mut to_remove = Vec::new();
			for (i, breakpoint) in self.breakpoints.iter().enumerate() {
				ui.horizontal(|ui| {
					ui.label(format!("{breakpoint}"));
					if ui.button("X").clicked() {
						to_remove.push(i);
					}
				});
			}
			to_remove.iter().for_each(|i| {
				self.breakpoints.remove(*i);
			});
		});
		egui::Window::new("Watchpoints").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Address:");
				if ui
					.add(
						egui::TextEdit::singleline(&mut self.watchpoints_user_entry)
							.desired_width(40.)
							.hint_text("in hex"),
					)
					.lost_focus() || ui.button("Add").clicked()
				{
					if let Ok(line_number) = u16::from_str_radix(&self.watchpoints_user_entry, 16) {
						if !self.watchpoints.contains(&line_number) {
							self.watchpoints.push(line_number);
						}
					}
					self.watchpoints_user_entry.clear();
				}
			});
			let mut to_remove = Vec::new();
			for (i, &watchpoint) in self.watchpoints.iter().enumerate() {
				ui.horizontal(|ui| {
					ui.label(format!(
						"0x{watchpoint:x} => 0x{:02x}",
						mem.read_byte(watchpoint)
					));
					if ui.button("X").clicked() {
						to_remove.push(i);
					}
				});
			}
			to_remove.iter().for_each(|i| {
				self.watchpoints.remove(*i);
			});
		});
		if self.breakpoints.contains(&(current_line_number + 1)) {
			self.paused = true;
		}
	}
}
