## Build, strip, copy, and run

### Mac OS

Pre-requisite: musl-cross (Homebrew users: `brew install FiloSottile/musl-cross/musl-cross --without-x86_64 --with-arm-hf`)

https://github.com/FiloSottile/homebrew-musl-cross

```bash
# build for rpi zero 2 w
cargo build --release --target=arm-unknown-linux-musleabihf
# look at the size of the bin file
ls -lh target/arm-unknown-linux-musleabihf/release/rustgotchi
# strip it
arm-linux-musleabihf-strip target/arm-unknown-linux-musleabihf/release/rustgotchi
# look at it now ;)
ls -lh target/arm-unknown-linux-musleabihf/release/rustgotchi
# copy over ssh
scp target/arm-unknown-linux-musleabihf/release/rustgotchi martin@cozigotchi.local:~/
# ssh into the rpi to run it
ssh martin@cozigotchi.local
# run it
./rustgotchi
```

### Linux

Not tested. Follow this article [Raspberry Pi Zero Raspbian/Rust Primer](https://dev.to/jeikabu/raspberry-pi-zero-raspbian-rust-primer-3aj6).
