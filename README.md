# rust-yt-tools

Experimental rust-yt work to compile to wasm.

To get this working, you will need to install [rustup](https://rustup.rs/) and
potentially set it to use the nightly installation.

The easiest way to build this package is to install the crate wasm-pack. Once
you've done that, you should be able to build from source using the following
steps:

```
git clone https://github.com/data-exp-lab/rust-yt-tools
cd ./rust-yt-tools
wasm-pack init --scope data-exp-lab
```
