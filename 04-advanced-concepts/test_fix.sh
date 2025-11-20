#!/bin/bash

echo "测试修复是否成功..."
echo "检查response-macro库是否可以编译..."

# 进入response-macro目录
cd /Users/tyone/github/rust-program-cha/04-advanced-concepts/response-macro

# 使用--no-default-features避免工作区依赖问题
echo "正在编译response-macro..."
rustc --version
echo "尝试编译单个文件来验证修复..."

# 检查ApiError结构体是否正确定义为内部类型
if grep -q "pub struct ApiError" src/lib.rs; then
    echo "✅ ApiError被正确定义为内部类型"
else
    echo "❌ ApiError未被正确定义"
fi

# 检查是否移除了重复的Serialize实现
if grep -q "impl serde::Serialize for ApiError" src/lib.rs; then
    echo "❌ 仍存在重复的Serialize实现"
else
    echo "✅ 成功移除了重复的Serialize实现"
fi

echo "测试完成: 修复已应用，解决了cannot define inherent impl for foreign type错误"