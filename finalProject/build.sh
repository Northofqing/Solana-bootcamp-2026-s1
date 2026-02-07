#!/bin/bash
set -e

echo "🔨 正在编译Pinocchio套利程序..."

# 清理之前的构建
cargo clean

# 编译release版本
echo "📦 编译release版本..."
cargo build --release --target-dir target

# 创建deploy目录
mkdir -p target/deploy

# 复制.so文件（在macOS上是.dylib）
if [ -f "target/release/libpinocchio_arbitrage.dylib" ]; then
    echo "📋 复制dylib为.so格式..."
    cp target/release/libpinocchio_arbitrage.dylib target/deploy/pinocchio_arbitrage.so
    echo "✅ 成功生成: target/deploy/pinocchio_arbitrage.so"
    ls -lh target/deploy/pinocchio_arbitrage.so
elif [ -f "target/release/libpinocchio_arbitrage.so" ]; then
    cp target/release/libpinocchio_arbitrage.so target/deploy/pinocchio_arbitrage.so
    echo "✅ 成功生成: target/deploy/pinocchio_arbitrage.so"
    ls -lh target/deploy/pinocchio_arbitrage.so
else
    echo "❌ 错误: 找不到编译输出文件"
    exit 1
fi

echo ""
echo "📊 文件信息:"
file target/deploy/pinocchio_arbitrage.so

echo ""
echo "🎉 编译完成！"
echo ""
echo "下一步："
echo "1. 使用 'solana program deploy target/deploy/pinocchio_arbitrage.so' 部署到Solana"
echo "2. 或者使用 'cargo build-sbf' 编译BPF版本（需要Solana工具链）"
