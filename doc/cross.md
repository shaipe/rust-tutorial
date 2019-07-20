rust 跨平台编译
===

在mac系统下编译到windows中一直还没有成功,还待研究.

## 环境配置
### 先添加需要编译的平台目录

```bash
# 1. 查看提供的所有平台
rustup target list 

# 2. 添加目标平台到编译器, 以常用的为例
# 基于linux内核的多平台 x86_64-unknown-linux-musl
# windows 平台 x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-musl


```
### mac上安装编译条件

```
brew install filosottile/musl-cross/musl-cross

brew install mingw-w64
```

### 添加配置

在 ``~/.cargo/config`` 中添加

```conf

# linux 平台

[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

# windows 平台
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

## 编译

### 编译到linux

CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl


## 参考文档

[rust-cross](https://github.com/japaric/rust-cross)
[交叉编译linux_musl](https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/)
[https://rendered-obsolete.github.io/2019/03/19/rust_lambda.html](https://rendered-obsolete.github.io/2019/03/19/rust_lambda.html)
[超方便的 rust 交叉编译](https://moevis.github.io/cheatsheet/2018/08/18/%E8%B6%85%E6%96%B9%E4%BE%BF%E7%9A%84-Rust-%E4%BA%A4%E5%8F%89%E7%BC%96%E8%AF%91.html)

[https://chr4.org/blog/2017/03/15/cross-compile-and-link-a-static-binary-on-macos-for-linux-with-cargo-and-rust/]
https://wiki.archlinux.org/index.php/Rust
https://hub.docker.com/r/ethankhall/rust-cross-build/