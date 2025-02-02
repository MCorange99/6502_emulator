# 6502 Emulator

Emulates a MOS 6502 chip with DMA graphics and input capibalities with interactive debugging.  
**[LIVE DEMO](https://archusr64.github.io/6502_emulator/)**

https://github.com/ArchUsr64/6502_emulator/assets/83179501/e0dbec0d-e08e-4925-af7d-f378ba173d0e

Source Code for [snake](examples/snake.asm)

## Build and Execution
1. Clone the repository:  
   `git clone https://github.com/ArchUsr64/6502_emulator`
2. Change to newly created directory:  
   `cd 6502_emulator`
4. Assemble one of the provided examples under `examples/` using [vasm](http://www.compilers.de/vasm.html) or just use the provided python build script:  
   `python build_asm.py examples/snake.asm`  
   This should build an `a.out` binary which the emulator can understand along with `symbols.dbg` for debugging.
5. Run the emulator:  
   `cargo run -- a.out --assembly-source examples/snake.asm --debug-symbols symbols.dbg`
6. For an explaination of all possible arguments:  
   `cargo run -- --help`

## Usage
### Debugging
Click the 'Pause Execution' button in the Debug Controls window to pause the execution at any time or start in paused state via the `-s` flag.
Once paused, use the `Step` button to execute the next instruction.

Add breakpoints from the 'Breakpoints' window and press the 'X' button to remove previously added entires.

Additionally the simulation speed can be adjusted using the slider or with the `-i` flag.

### Logging
Use the `-v` flag to specify the level of verbosity for log output:
| `-v` | Log Level |
| -- | -- |
| 0 | Error |
| 1 | Info |
| 2 | Debug |
### Inputs
Currently only four inputs are supported, `LEFT`, `DOWN`, `UP` and `RIGHT`. Both the arrow keys and the WASD cluster can be used to activate their respective inputs.

## Memory Layout  
| Address | Description |
| -- | -- |
| `0xfb - 0xfe` | Keyboard Inputs stored here in: `left`, `down`, `up`, `right` order where 1 indicates `KeyDown` |
| `0xff` | Random Number Generator (Value is updated to a random byte on every instruction execution |
| `0x100 - 0x1ff` | Stack to store subroutine return addresses |
| `0xfb00 - 0xffff` | `0x400` (1024) byte space to store the RGB values for pixels on a 32x32 grid in standard raster scan order | 

### RGB color format:
Each color byte is divided into bit fields of size 3, 3 and 2. The bit field if size 2 is least significant and represents the blue color, with the most significant bit field representing red as shown below:
```
MSB      LSB
 ^        ^
 765 432 10
 |   |   |
 RRR GGG BB
```

## Screenshots
#### [examples/snake.asm](examples/snake.asm)  
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/2c104306-8233-4f13-9b76-5ee321c4d05f)
<br>
#### [examples/rgb.asm](examples/rgb.asm)  
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/9a6a5d93-d806-431a-af00-5bded1c93793)  
<br>
#### [examples/ferris.asm](examples/ferris.asm)  
![image](https://github.com/ArchUsr64/6502_emulator/assets/83179501/8fcb2804-92d0-43a3-abd1-ef00b96d773d)
