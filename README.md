The code should be commented fairly well. Just look at the code if you want to understand something. It's written in rust so you will need at least rustc, preferably cargo.

I consider this now v1.0.0 because repack produces an identical file from an unpacked directory. At least on the files I tested.

## Running
Clone the repo, build, run:

```
git clone https://github.com/quvide/mpk-tools.git
cd mpk-tools
cargo build
./target/debug/mpk-tools unpack -o [target dir] [source file]
```
The unpacker prefixes every file with the index to preserve order when repacking. I'm not sure if it's actually needed.

To repack, run
```
./target/debug/mpk-tools pack -o [target file] [source dir]
```
