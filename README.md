# drop

## Building

Required tools installed on the system path:
cargo
strip
sstrip (from elfkickers)
upx

```
git clone https://github.com/vess-dev/drop
cd drop
chmod +x build.sh
./build.sh
target/release/drop
```

## Help

usage:

drop chance: f64 = (0, 100)
chests: u64 = (0, max]
trials: u64 = (0, max]

drop <drop chance> <chests> <trials>
	Check the total chance out of N trials to recieve 1 item out of N chests if the item has N drop chance.

Example usage:
	drop 4.55 20 10000000

## License

https://creativecommons.org/licenses/by/4.0/