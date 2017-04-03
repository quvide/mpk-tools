Clone the repo, build, run:

```
git clone https://github.com/quvide/mpk-tools.git
cd mpk-tools
cargo build
target/debug/mpk-tools unpack -o [target dir] [source file]
```
The unpacker prefix every file with the index to preserve order when repacking. I'm not sure if it's actually needed.

Repacking is not yet supported.
