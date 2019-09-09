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


### 采用docker实现跨平台编译

#### 1. 创建和进入容器
```bash
# 创建docker
docker pull centos

# 运行容器对象
docker run -itd \
-p 8090:80 \
--name rust-centos \
--restart always \
-v /Users/shaipe/workspace/rust/:/data \
shaipe/rust-centos

# 进入容器
docker exec -it rust-centos bash
```

#### 2. RUST环境搭建

```bash

# 安装vim
yum install vim

# 安装rust环境
curl https://sh.rustup.rs -sSf | sh

# 添加环境变量
# 临时环境变量
source $HOME/.cargo/env

# 永久环境变量
vim ~/.bashrc

# 添加Cargo环境变量到系统中
export PATH="$HOME/.cargo/bin:$PATH"

# 让配置的环境变量生效
source ~/.bashrc
# 设置centos的时区
cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
# 安装cc的编译环境
yum install gcc
# 使用到openssl的环境
yum install openssl-devel
```

#### 3. 提交docker

```bash
# 提交docker容器
docker commit -a "shaipe" -m "rust-coentos" 7c07b9fa7753 shaipe/rust-centos
# 登录容器环境
docker login 
# 推送容器到docker hub
docker push shaipe/rust-centos
```

## 参考文档

[rust-cross](https://github.com/japaric/rust-cross)
[交叉编译linux_musl](https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/)
[https://rendered-obsolete.github.io/2019/03/19/rust_lambda.html](https://rendered-obsolete.github.io/2019/03/19/rust_lambda.html)
[超方便的 rust 交叉编译](https://moevis.github.io/cheatsheet/2018/08/18/%E8%B6%85%E6%96%B9%E4%BE%BF%E7%9A%84-Rust-%E4%BA%A4%E5%8F%89%E7%BC%96%E8%AF%91.html)

[https://chr4.org/blog/2017/03/15/cross-compile-and-link-a-static-binary-on-macos-for-linux-with-cargo-and-rust/]
https://wiki.archlinux.org/index.php/Rust
https://hub.docker.com/r/ethankhall/rust-cross-build/