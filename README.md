# ZeroHasher

**ZeroHasher** searches among non-negative numbers `F` numbers whose hash (obtained using SHA-256) contains `N` zeros at the end.

## How to build?

To build and run **ZeroHasher** you need to install [Rust](https://www.rust-lang.org/) and git.

Clone repo and enter it:

```
git clone https://github.com/spireneko/PyShop-internship.git
cd PyShop-internship
```

Run the `cargo` command - it will download all dependencies, compile the code, and run the program with the provided parameters:
```
cargo run --release -- -N 3 -F 6
```