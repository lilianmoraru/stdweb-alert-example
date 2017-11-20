# Rust stdweb alert example

### Building and running:
First, install [`wargo`](https://github.com/lord/wargo).
Then follow these steps:
```
$ wargo build --release
$ cargo +nightly run --release
```
You should see a simple input field in which you can insert text and then press `Enter`,
after which an `alert` should appear with that text.
