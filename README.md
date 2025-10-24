# Packet Tracer TL;DR

> Inspired from [tldr](https://github.com/tldr-pages/tldr).

It doesn't work in Packet Tracer,
but is used on Linux to summarize pkt commands.


## How to install on Linux

### Option 1: Manual Build

Make sure you have rustc/cargo already installed:
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
$ cargo build
$ sudo cp ./target/debug/tldr-pkt /usr/bin
$ reset
```

Now, you can run the CLI tool by simply:
```sh
$ tldr-pkt
```


