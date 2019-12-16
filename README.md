RUST Lang
===

## 包管理辅助工具

可以把包下载到本地进行管理,就不需要每次都从外网下载,可以实现半个离线包管理吧!

* [cargo-edit](https://crates.io/crates/cargo-edit)


## 参考文档

### Web 框架

- [Nickel](http://nickel-org.github.io/)
- [Actix](https://actix.rs)

### 交叉编译

* [交叉编译linux_musl](https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/)
* [rust_lambda 表达式](https://rendered-obsolete.github.io/2019/03/19/rust_lambda.html)
* [超方便的 rust 交叉编译](https://moevis.github.io/cheatsheet/2018/08/18/%E8%B6%85%E6%96%B9%E4%BE%BF%E7%9A%84-Rust-%E4%BA%A4%E5%8F%89%E7%BC%96%E8%AF%91.html)

```bash
# 跨平台编译时,需要指定环境变量
# 主要是在使用的ssl的地方,在linux下使用的是openssl而windows天macos是使用的系统自带的ssl
OPENSSL_STATIC=1 \
OPENSSL_LIB_DIR=/usr/local/cellar/openssl@1.1/1.1.1c/lib \
OPENSSL_INCLUDE_DIR=/usr/local/cellar/openssl@1.1/1.1.1c/include \
CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" \
cargo build --target=x86_64-unknown-linux-musl
```

### Rust WebAssembly

* [WebAssembly + Rust 上手初探](https://www.codercto.com/a/43181.html)
* [Rust 和 WebAssembly 用例](https://developer.mozilla.org/zh-CN/docs/WebAssembly/Rust_to_wasm)

### 其他

* [Rust学习笔记 - 模块系统](https://www.codercto.com/a/84199.html)

### 电子书

* [Rust 程序设计语言（第二版） 简体中文版](https://kaisery.gitbooks.io/trpl-zh-cn/content/ch01-00-getting-started.html)
* [Cargo](https://www.mankier.com/package/cargo)


### 库

* [docs.rs](https://docs.rs/)
* [Crates.io](https://crates.io/)


### 在Ubuntu下安装rust

```bash
# 安装好rust环境后需要执下面的语句安装编译
apt install build-essential
# openssl编译错误  HTTP
# failed to run custom build command for `openssl-sys v0.9.53`
apt install pkg-config
apt install libssl-dev
```