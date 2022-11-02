.PHONY: setup image qemu
.EXPORT_ALL_VARIABLES:

setup:
	curl https://sh.rustup.rs -sSf | sh -s -- -y
	rustup install nightly
	rustup default nightly
	cargo install bootimage

# Compilation options
output = video# video, serial
keyboard = qwerty# qwerty, azerty, dvorak
mode = release

# Emulation options
nic = rtl8139# rtl8139, pcnet
audio = sdl# sdl, coreaudio
kvm = false
pcap = false

export sspos_KEYBOARD = $(keyboard)

# Build userspace binaries
user-nasm:
	basename -s .s dsk/src/bin/*.s | xargs -I {} \
    nasm dsk/src/bin/{}.s -o dsk/bin/{}.tmp
	basename -s .s dsk/src/bin/*.s | xargs -I {} \
		sh -c "printf '\x7FBIN' | cat - dsk/bin/{}.tmp > dsk/bin/{}"
	rm dsk/bin/*.tmp
user-rust:
	basename -s .rs src/bin/*.rs | xargs -I {} \
		touch dsk/bin/{}
	basename -s .rs src/bin/*.rs | xargs -I {} \
		cargo rustc --release --bin {}
	basename -s .rs src/bin/*.rs | xargs -I {} \
		cp target/x86_64-sspos/release/{} dsk/bin/{}
	#strip dsk/bin/*

bin = target/x86_64-sspos/$(mode)/bootimage-sspos.bin
img = disk.img

$(img):
	qemu-img create $(img) 32M


cargo-opts = --no-default-features --features $(output) --bin sspos
ifeq ($(mode),release)
	cargo-opts += --release
endif

# Rebuild sspos if the features list changed
image: $(img)
	touch src/lib.rs
	env | grep sspos
	cargo bootimage $(cargo-opts)
	dd conv=notrunc if=$(bin) of=$(img)


qemu-opts = -m 32 -drive file=$(img),format=raw \
			 -audiodev $(audio),id=a0 -machine pcspk-audiodev=a0 \
			 -netdev user,id=e0,hostfwd=tcp::8080-:80 -device $(nic),netdev=e0
ifeq ($(kvm),true)
	qemu-opts += -cpu host -accel kvm
else
	qemu-opts += -cpu max
endif

ifeq ($(pcap),true)
	qemu-opts += -object filter-dump,id=f1,netdev=e0,file=/tmp/qemu.pcap
endif

ifeq ($(output),serial)
	qemu-opts += -display none -chardev stdio,id=s0,signal=off -serial chardev:s0
endif

ifeq ($(mode),debug)
	qemu-opts += -s -S
endif

# In debug mode, open another terminal with the following command
# and type `continue` to start the boot process:
# > gdb target/x86_64-sspos/debug/sspos -ex "target remote :1234"

qemu:
	qemu-system-x86_64 $(qemu-opts)

test:
	cargo test --release --lib --no-default-features --features serial -- \
		-m 32 -display none -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04

clean:
	cargo clean
