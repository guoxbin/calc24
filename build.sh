#!/bin/bash

# 构建脚本 - 构建前端静态文件和后端可执行文件
#
# 使用方法:
#   ./build.sh        # 仅构建前端（推荐，在服务器上编译后端）
#   ./build.sh --all  # 构建前端和后端（仅在 Linux 上使用）

set -e  # 遇到错误立即退出

echo "🔨 开始构建算24点项目..."

# 1. 构建前端
echo ""
echo "📦 步骤 1/2: 构建前端静态文件..."
cd web
npm install
npm run build
cd ..

# 2. 构建后端
echo ""
echo "🦀 步骤 2/2: 编译 Rust 后端..."

# 检测操作系统
OS=$(uname -s)

if [ "$1" = "--all" ]; then
    # 构建后端
    if [ "$OS" = "Darwin" ]; then
        echo "⚠️  警告: 在 Mac 上编译的是 macOS 版本，不能在 Linux 上运行"
        echo "如需 Linux 版本，请在 Linux 服务器上运行此脚本"
        echo ""
    fi
    
    echo "编译 Rust 后端..."
    cargo build --release
    BINARY_PATH="target/release/calc24"
    
    echo ""
    echo "✅ 构建完成！"
    echo ""
    echo "📁 构建产物："
    echo "  - 可执行文件: $BINARY_PATH"
    echo "  - 静态文件: web/dist/"
else
    # 默认：仅构建前端
    echo "⏭️  跳过后端编译（默认模式）"
    echo ""
    echo "✅ 前端构建完成！"
    echo ""
    echo "📁 构建产物："
    echo "  - 静态文件: web/dist/"
    echo ""
    echo "💡 部署到 Linux 服务器："
    echo "  1. 上传项目: scp -r . user@server:/opt/calc24"
    echo "  2. SSH 登录: ssh user@server"
    echo "  3. 编译后端: cd /opt/calc24 && cargo build --release"
    echo "  4. 运行服务: ./target/release/calc24"
    echo ""
    echo "💡 或使用部署脚本（需先在服务器上编译）："
    echo "  ./deploy.sh user@server:/opt/calc24"
fi
