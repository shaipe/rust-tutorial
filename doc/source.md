Rust Cargo 安装源替换

## cargo 配置

使用说明
    在 `$HOME/.cargo/config` 中添加如下内容：

```yaml
[source.crates-io]
replace-with = 'ustc'
# source
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

如果所处的环境中不允许使用 git 协议，可以把上述地址改为：

```yaml
registry = "https://mirrors.ustc.edu.cn/crates.io-index"
```