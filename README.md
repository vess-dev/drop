# `drop`

![Example](example.png)

## Help with `drop`

```
usage:

drop chance: f32 = (0.##, 100.##)
chests: u32 = (0, 4,294,967,295]
trials: u32 = (0, 4,294,967,295]

drop <drop chance> <chests> <trials>
	Simulate the chance out of N trials to get at least 1 item from N chests if the item has N drop chance.

Example usage:
	drop 4.55 20 1,000,000
```

## Running the program, `debug build`

Required tools installed on the system path:
- cargo and a Rust toolchain installed
- git

```
git clone https://github.com/vess-dev/drop
cd drop
cargo run
```

## Compiling a _large_ `release build`

Required tools installed on the system path:
- cargo and a Rust toolchain installed
- git

```
git clone https://github.com/vess-dev/drop
cd drop
cargo build --release
target/release/drop
```

## Compiling a _small_ `release build`

Required tools installed on the system path:
- cargo and a Rust toolchain installed
- git
- strip
- sstrip (from elfkickers)
- upx

```
git clone https://github.com/vess-dev/drop
cd drop
chmod +x build.sh
./build.sh
target/release/drop
```

## Crosscompile a _small_ `Windows build`

Required tools installed on the system path:
- cargo and a Windows Rust target installed
- git
- strip
- upx

Warning: Crosscompilation for Windows is <u>broken</u> on some Linux distributions.

How to fix: https://wiki.archlinux.org/title/rust#Windows

```
git clone https://github.com/vess-dev/drop
cd drop
cargo build --release --target x86_64-pc-windows-gnu
strip target/x86_64-pc-windows-gnu/release/drop.exe
upx --lzma target/x86_64-pc-windows-gnu/release/drop.exe
```

## License

https://creativecommons.org/licenses/by/4.0/