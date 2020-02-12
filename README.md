# gtrans
Google translation CLI tool written in Rust

<div align="center">
    <img src="images/screen_shot.png" title="screen shot">
</div>

## install

```bash
$ cargo install gtrans
```

In macOS you can install it with Homebrew
```bash
$ brew tap ksk001100/homebrew-gtrans
$ brew install gtrans
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