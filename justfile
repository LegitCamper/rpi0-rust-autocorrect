KERNEL_BIN        := "kernel.img"
OBJDUMP_BINARY    := "arm-none-eabi-objdump"
NM_BINARY         := "arm-none-eabi-nm"
READELF_BINARY    := "arm-none-eabi-readelf"
LD_SCRIPT_PATH    := "./linker.ld"
KERNEL_MANIFEST   := "Cargo.toml"
LAST_BUILD_CONFIG := "target/RPI0.build_config"
KERNEL_ELF        := "target/thumbv6m-none-eabi/release/kernel"
QEMU_ARGS         := "-machine type=raspi0 -m 512 -nographic -kernel"

default: img

##------------------------------------------------------------------------------
## Compile the kernel ELF
##------------------------------------------------------------------------------
elf: clean
	echo "Compiling kernel ELF - RPI0"
	cargo build --release

##------------------------------------------------------------------------------
## Generate the stripped kernel binary
##------------------------------------------------------------------------------
img: elf
	echo "Generating stripped binary"
	arm-none-eabi-objcopy {{KERNEL_ELF}} -O binary kernel.img
	echo {{KERNEL_BIN}}

##------------------------------------------------------------------------------
## Run in qemu
##------------------------------------------------------------------------------
qemu: 
	qemu-system-arm {{QEMU_ARGS}} {{KERNEL_ELF}}

##------------------------------------------------------------------------------
## Run clippy
##------------------------------------------------------------------------------
clippy:
	cargo clippy

##------------------------------------------------------------------------------
## Clean
##------------------------------------------------------------------------------
clean:
	rm -rf target {{KERNEL_BIN}}

##------------------------------------------------------------------------------
## Run readelf
##------------------------------------------------------------------------------
readelf: elf
	@echo "Launching readelf"
	arm-none-eabi-readelf --headers {{KERNEL_ELF}}

##------------------------------------------------------------------------------
## Run objdump
##------------------------------------------------------------------------------
objdump: elf
	@echo "Launching objdump"
	arm-none-eabi-objdump {{KERNEL_ELF}} -D | less

##------------------------------------------------------------------------------
## Run nm
##------------------------------------------------------------------------------
nm: elf
	@echo "Launching nm"
	{{NM_BINARY}} "--demangle --print-size {{KERNEL_ELF}} | sort | rustfilt"
