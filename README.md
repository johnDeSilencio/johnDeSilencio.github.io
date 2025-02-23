# My Website

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/MIT)
[![codecov](https://codecov.io/gh/johnDeSilencio/johnDeSilencio.github.io/graph/badge.svg?token=EB7LWX199G)](https://codecov.io/gh/johnDeSilencio/johnDeSilencio.github.io)
![e2e-tests](https://github.com/johnDeSilencio/johnDeSilencio.github.io/actions/workflows/e2e-tests.yaml/badge.svg)
![MSRV](https://img.shields.io/badge/MSRV-1.85.0-crimson)

A personal website showcasing my resume, personality, and skillset.

## Installation

Prerequisites:

1. You have `rustc` and `cargo` installed on your computer (you can follow the
   guide [here](https://www.rust-lang.org/tools/install))
2. You have installed `trunk`: `cargo install --locked trunk`
3. You have installed the `wasm32-unknown-unknown` target:
   `rustup install wasm32-unknown-unknown`
4. You have cloned the repository

```none
$> git clone https://github.com/johnDeSilencio/johnDeSilencio.github.io.git
```

## Usage

Navigate to the directory where you have cloned the repository and run the
following command:

```none
$> trunk serve
```

After `trunk` finishes building, navigate to `http://127.0.0.1:8080` to view the
website locally.

## FAQ

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
