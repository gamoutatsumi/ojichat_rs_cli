# ojichat_rs_cli

[![Crates.io](https://img.shields.io/crates/v/ojichat-cli)](https://crates.io/crates/ojichat-cli)
[![MIT LICENSE](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gamoutatsumi/ojichat_rs_cli/Build%20and%20Publish%20release)](https://github.com/gamoutatsumi/ojichat_rs_cli/actions?query=workflow:%22Build+and+Publish+release%22)

[ojichat_rs](https://github.com/gamoutatsumi/ojichat_rs)のCLI版です。

## 開発環境

```bash
$ rustup default
nightly-x86_64-unknown-linux-gnu (default)
```

## インストール

```bash
cargo install ojichat-cli
```

## 使い方

```bash
$ ojichat-rs -h
ojichat-cli 0.1.0
Tatsumi Gamou <tatsumigamou@yahoo.co.jp>
Ojisan Nanchatte (ojichat) command

USAGE:
    ojichat-rs [OPTIONS] [name]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e <number>        絵文字/顔文字の最大連続数 [default: 4]
    -p <level>         句読点挿入頻度レベル [min:0, max:3] [default: 0]

ARGS:
    <name>    おじさんのメール相手
```

基本的な使い方は[本家](https://github.com/greymd/ojichat)と同じです。

## ライセンス

[MIT](./LICENSE)
