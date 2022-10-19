RandStrGen is a Command-Line tool for generating strings of some length, from a pool of characters.

## Compiling
A built binary for MacOS is at `./rand-str-gen`.

Otherwise, you can compile the source code with
```bash
git clone "https://github.com/MasonFeurer/RandStrGen.git"
cd RandStrGen
cargo build --release
```
And an executable will be placed in `./target/release`.

## Using
For help with optional and/or required arguments, use
```bash
rand-str-gen --help
```

Generate a string of length 10
```bash
rand-str-gen 10
```

Then exclude misc symbols
```bash
rand-str-gen 10 -m
```

Then add characters to pool: '%', '$', '^', '@'
```bash
rand-str-gen 10 "+[%$^@]"
```

Use default sets, but without '.'
```bash
rand-str-gen 10 "-[.]"
```
