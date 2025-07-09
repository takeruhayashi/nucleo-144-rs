# nucleo-144-rs
STM32 Nucleo-144 Microcontroller Board Software with Embassy Crate Framework

# Test Environment
* OS: Ubuntu 22.04.5 LTS
* Kernel: 6.8.0-60-generic x86_64 GNU/Linux

# Rust Toolchain Setup
1. Install Rust
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Check Rust compiler version (it should be equal to or newer than `1.31`)
```
$ rustc -V
rustc 1.87.0 (17067e9ac 2025-05-09)
```
3. Add cross compilation target for Cortex-M33
```
$ rustup target add thumbv8m.main-none-eabihf
```
4. Install cargo-binutils
```
$ cargo install cargo-binutils
```
5. Add llvm-tools
```
$ rustup component add llvm-tools
```
6. Install cargo-generate
```
$ cargo install cargo-generate
```
7. Install dependencies for Ubuntu 18.04 or newer
```
$ sudo apt install libudev-dev gdb-arm-none-eabi openocd qemu-system-arm
```
8. Create the file `/etc/udev/rules.d/49-stlinkv3.rules` with the contents shown below for udev rules
```
# STLink V3SET in Dual CDC mode
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3752", MODE:="0666", SYMLINK+="stlinkv3_%n"

# STLink V3SET in Dual CDC mode
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3753", MODE:="0666", SYMLINK+="stlinkv3_%n"
    
# STLink V3SET 
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374d", MODE:="0666", SYMLINK+="stlinkv3_%n"

# STLink V3SET 
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374e", MODE:="0666", SYMLINK+="stlinkv3_%n"
    
# STLink V3SET in normal mode
SUBSYSTEMS=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374f", MODE:="0666", SYMLINK+="stlinkv3_%n"
```
9. Reload the udev rules
```
$ sudo udevadm control --reload-rules
$ sudo udevadm trigger
```
10. Check connected microcontroller
```
$ lsusb | grep -i stlink
```
which should show something like
```
Bus 003 Device 003: ID 0483:374e STMicroelectronics STLINK-V3
```
11. Install embedded toolkit (`probe-rs`)
```
cargo install probe-rs-tools
```

## Usage
### LED Example
```
cargo run --bin led
```
### Button Example
```
cargo run --bin button
```
### Ethernet Example
```
cargo run --bin ethernet
```
### TCP Echo Server Example
```
cargo run --bin echo_server
```
### ADC Example
```
cargo run --bin adc
```
### I2C Example
```
cargo run --bin i2c
```
### SPI Example
```
cargo run --bin spi
```