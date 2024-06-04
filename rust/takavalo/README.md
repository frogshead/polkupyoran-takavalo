# Usage

```rust
cargo build --release
```

Then follow the instructions.

~/ohjelmointi/github/polkupyoran-takavalo/rust/takavalo/target/thumbv6m-none-eabi/release> arm-none-eabi-objcopy -O binary takavalo takavalo.bin


~/ohjelmointi/github/polkupyoran-takavalo/rust/takavalo/target/thumbv6m-none-eabi/release> stm32flash -w takavalo.bin -v /dev/ttyUSB0
