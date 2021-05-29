# rust-domain.rs

![experimental](http://badges.github.io/stability-badges/dist/experimental.svg)

> I need a .rs domain, now!

## What does `rust-domain` do?

It helps you creating a `.rs` domain for your rust project from the shell.

## How does it work?

There is no magic, it relies on an ISP (their API) to register the domain. 

Right now only one ISP ([mCloud.rs](https://www.mcloud.rs)) is supported, for 2 reasons.
1. They offer a fair price (~15 €)
2. They offer an API to make this possible

Feel free to contribute to support other ISPs.

So to be clear, you need to signup with this ISP first (`rust-domain signup`) then you can go ahead.

## FAQ

> Q: So this tool is only an API wrapper?
> A: yes, but tries to be convenient.

> Q: Is there a Web-UI?
> A: No, but you can go to the ISP and do it all there, so this tool is not needed for you then.

## Quick Start
### Install

To install the rust-domain, you just need to run

```bash
cargo install rust-domain
```

to verify if the installation was successful, you can run `which rust-domain` that should output similar to

```sh
$HOME/.cargo/bin/rust-domain
```

### Usage

```sh
$ rust-domain --help

TODO: --help output goes here
```

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

- **[GNU GPL v3 license](https://www.gnu.org/licenses/gpl-3.0)**
- Copyright 2021 © [Sven Assmann][me].

[me]: https://www.d34dl0ck.me
