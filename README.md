# sos
4fun operating system  

Currently almost entirely following along with [os,phil-opp](https://os.phil-opp.com)


#### Description

###### Booting
The bootloader crate is used as the bootloader for this project. It implements a basic BIOS
bootloader without relying on C dependencies.

The bootimage tool is responsible of linking the bootloader and the kernel into a bootable disk
image. It compiles the kernel into an elf file, compiles the bootloader dependency into a standalone
executable, and then proceeds to link the kernel's elf file and the bootloader together.

#### Installation
```
rustup toolchain install nightly
cargo install bootimage
rustup component add llvm-tools-preview
```

#### Usage
```
cargo run
```

