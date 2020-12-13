# ojichat_rs_cli

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
    -e <number>        絵文字/顔文字の最大連続数 [default: 4] [default: 4]
    -p <level>         句読点挿入頻度レベル [min:0, max:3] [default: 0] [default: 0]

ARGS:
    <name>    おじさんのメール相手
```

基本的な使い方は[本家](https://github.com/greymd/ojichat)と同じです。

## ライセンス

[MIT](./LICENSE)
