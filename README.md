    $ git clone https://github.com/denver-code/sspos
    $ cd sspos

Install the required tools with `make setup` or the following commands:

    $ curl https://sh.rustup.rs -sSf | sh
    $ rustup install nightly
    $ rustup default nightly
    $ cargo install bootimage


## Usage

Build the image to `disk.img`:

    $ make image output=video keyboard=qwerty

Run sspos in QEMU:

    $ make qemu output=video nic=rtl8139

Run natively on a x86 computer by copying the bootloader and the kernel to a
hard drive or USB stick (but there is currently no USB driver so the filesystem
will not be available in that case):

    $ sudo dd if=target/x86_64-sspos/release/bootimage-sspos.bin of=/dev/sdx && sync

sspos will open a console in diskless mode after boot if no filesystem is
detected. The following command will setup the filesystem on a hard drive,
allowing you to exit the diskless mode and log in as a normal user:

    > install

**Be careful not to overwrite the hard drive of your OS when using `dd` inside
your OS, and `install` or `disk format` inside sspos if you don't use an
emulator.**


## Tests

Run the test suite in QEMU:

    $ make test


## License

sspos is released under MIT.
