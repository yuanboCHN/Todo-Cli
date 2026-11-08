# todo-cli

一个用 Rust 写的命令行待办管理工具，支持本地 JSON 持久化。

## 使用

### 安装（任意目录）

```bash
cargo install --path .
todo-cli add "买牛奶"
todo-cli list
todo-cli done 1
todo-cli delete 1
```

示例输出：

```
$ cargo run -- list
[ ] 2. 买猪肉
[x] 1. 买牛奶
```
