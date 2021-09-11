# sabi

In Japanese version https://github.com/bnjbvr/rouille. Shamelessly copied and updated from it.

日本語で Rust プログラムを書くことができます！

# 例

`main.rs`

```rust
sabi::sabi! {
    外部 クレート sabi;

    関数 メイン() {
        書き出す!("こんにちは、世界");
    }
}
```

出力結果。

```
❯ cargo run
こんにちは、世界
```

# Thank you

https://github.com/bnjbvr/rouille

# License

[WTFPL](http://www.wtfpl.net/)
