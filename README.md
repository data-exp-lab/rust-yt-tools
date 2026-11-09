# rust-yt-tools

Experimental rust-yt work to compile to wasm.

To get this working, you will need to install [rustup](https://rustup.rs/)
(stable Rust is fine) and add the wasm target with
`rustup target add wasm32-unknown-unknown`.

The easiest way to build this package is to install the crate wasm-pack. Once
you've done that, you should be able to build from source using the following
steps:

```
git clone https://github.com/data-exp-lab/rust-yt-tools
cd ./rust-yt-tools
wasm-pack build --scope data-exp-lab
```

This package is primarily designed as a mechanism for creating wasm code to be
used in Jupyter widgets, for instance as in the
[widgyts](https://github.com/yt-project/widgyts/) library, but may grow to
include more functionality.

## Releasing

Releases are published to npm as `@data-exp-lab/yt-tools` by the
`npm-publish.yml` workflow, using npm trusted publishing (no npm token is
stored in the repository).

1. Bump `version` in `Cargo.toml` and merge that change to `main`.
2. Create a GitHub release from `main` with the tag `vX.Y.Z`, matching the
   `Cargo.toml` version. The workflow refuses to publish if they differ.
3. Publishing the release triggers the workflow, which tests, builds the
   scoped package with `wasm-pack`, and runs `npm publish`.
