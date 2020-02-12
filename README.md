# gtrans
Google translation CLI tool written in Rust

## install

```bash
$ cargo install gtrans
```

## usage

Default translation source is `ja` and target is `en`.

The source and target can be changed by setting the environment variables `GTRANS_SOURCE` and `GTRANS_TARGET`.

```bash
$ gtrans こんにちは世界
$ export GTRANS_SOURCE=en
$ export GTRANS_TARGET=ja
$ gtrans Hello world
$ GTRANS_SOURCE=en GTRANS_TARGET=ja gtrans Hello world
```