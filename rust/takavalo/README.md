# Usage

```rust
cargo build --release
```

Then follow the instructions.

~/ohjelmointi/github/polkupyoran-takavalo/rust/takavalo/target/thumbv6m-none-eabi/release> arm-none-eabi-objcopy -O binary takavalo takavalo.bin


~/ohjelmointi/github/polkupyoran-takavalo/rust/takavalo/target/thumbv6m-none-eabi/release> stm32flash -w takavalo.bin -v /dev/ttyUSB0

Then follow the instructions

After flashing first time, the MCU does not go to bootloader mode where it tries to boot from the system memory and waiting for commands to serial port. 

nBOOT1 bit BOOT0 pin nBOOT_SEL bit nBOOT0 bit
  1           1         0              x           System memory

  