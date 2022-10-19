RandStrGen is a Command-Line tool for generating strings of some length, from a pool of characters.

## Compiling
A built binary for MacOS is at `/rand-str-gen`.

Otherwise, you can compile the source code with
```bash
git clone "https://github.com/MasonFeurer/RandStrGen.git"
cd RandStrGen
cargo build --release
```
Once built, you can run it with
```bash
./target/release/rand-str-gen
```

## Using
For help with optional and/or required arguments, use
```bash
rand-str-gen --help
```

To generate a string of length 10, use
```bash
rand-str-gen 10
```

To exclude symbols, use
```bash
rand-str-gen 10 -s
```
