# Tauri + React

This template should help get you started developing with Tauri and React in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 学习文章地址

https://juejin.cn/post/7067342513920540686#heading-8

## 打包

```ps
yarn tauri build
```

该命令会将 Web 资源 与 Rust 代码一起嵌入到单个二进制文件中。

1. 二进制文件本身将位于 src-tauri/target/release/[app name]
2. 安装程序将位于 src-tauri/target/release/bundle/
