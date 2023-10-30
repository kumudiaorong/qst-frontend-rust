# qst-frontend-rust

## Introduction

This is the frontend of [qst](https://github.com/kumudiaorong/qst-grpc) written in rust.
## Getting Started

### Get Release

No release yet.

### Build from Source

#### Dependencies

First please install [rust](https://www.rust-lang.org/tools/install).

Then see [tonic](https://github.com/hyperium/tonic) for other dependencies.

#### Get Source

```bash
git clone https://github.com/kumudiaorong/qst-frontend-rust.git
git submodule update --init
```

#### Build

```bash
cargo b --bin qst-f --release
```
The binary will be in `target/release/qst-f`.