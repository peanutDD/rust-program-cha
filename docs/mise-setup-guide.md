# Mise 设置指南

## 问题描述

当您在终端中看到以下错误信息时：

```
mise is not activated, run mise help activate or
read documentation at https://mise.jdx.dev for activation instructions.
Alternatively, add the shims directory ~/.local/share/mise/shims to PATH.
Using the shims directory is preferred for non-interactive setups.
```

这表示 Mise 版本管理器已安装但未正确激活。

## 解决方案

我们已经在项目根目录创建了一个 `.tool-versions` 文件，指定了项目使用的 Rust 版本：

```
rust 1.75.0
```

### 解决 "No such file or directory" 错误

如果您遇到类似以下错误：

```
mise ERROR Failed to install core:rust@1.75.0:
   0: failed write: ~/.local/share/mise/installs/rust/.mise.backend
   1: No such file or directory (os error 2)
```

这表示 Mise 的安装目录结构不存在。请执行以下命令创建必要的目录：

```bash
mkdir -p ~/.local/share/mise/installs/rust
```

### 解决激活问题的选项

要完全解决 Mise 激活问题，您有以下几个选项：

### 选项 1: 激活 Mise（推荐用于交互式环境）

根据您使用的 shell，运行相应的激活命令：

**对于 Bash 或 Zsh：**
```bash
eval "$(mise activate bash)"
# 或
eval "$(mise activate zsh)"
```

要使其永久生效，请将上述命令添加到您的 `~/.bashrc` 或 `~/.zshrc` 文件中。

### 选项 2: 添加 shims 目录到 PATH（推荐用于非交互式环境）

将 Mise shims 目录添加到您的 PATH 环境变量：

```bash
export PATH="$HOME/.local/share/mise/shims:$PATH"
```

同样，将此命令添加到您的 shell 配置文件中以使其永久生效。

## 验证安装

设置完成后，您可以通过以下命令验证 Mise 是否已正确配置：

```bash
mise --version
```

## 项目特定配置

项目中的 `.tool-versions` 文件指定了 Rust 1.75.0 作为项目使用的版本。当您在项目目录中运行 Rust 命令时，Mise 将确保使用正确的版本。

## 进一步阅读

- [Mise 官方文档](https://mise.jdx.dev/)
- [Mise 快速入门指南](https://mise.jdx.dev/getting-started.html)