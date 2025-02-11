# Usage
Original plan was to design a pcb to blink a led when ever motion is detected by the spring sensor.

Design goals were to have simple microcontroller [STM32G030F6Px](https://www.st.com/en/microcontrollers-microprocessors/stm32g030f6.html) which is supported by embedded rust ecosystem. Simple enough to design usign kicad by me. Try-out ordering from jlcpcb either only pcb or assembled pcb. 

## Things to learn
- Kicad 8
- JLCPCB kicad tools and ordering process
- Embassy embedded rust framework
- embedded-hal
- Low-power modes
- Battery powered

## Building the rust code
 

```rust
# Building the binary
cargo build --release
# Building, programming and running using probe-rs
# See the configuration: .cargo/config.toml
cargo run --release 
```
To pcb version 2 the debugger connector is added and pin out is for [rusty-probe](https://github.com/probe-rs/rusty-probe)
