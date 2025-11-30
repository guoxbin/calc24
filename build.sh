#!/bin/bash

# 构建脚本 - 用于构建 Linux 可执行文件和前端静态文件
#
# 使用方法:
#   ./build.sh          # 自动检测系统并构建
#   ./build.sh --local  # 仅构建前端，跳过后端编译（在服务器上编译）
#   ./build.sh --docker # 强制使用 Docker 构建

set -e  # 遇到错误立即退出

# 解析参数
BUILD_MODE="auto"
if [ "$1" = "--local" ]; then
    BUILD_MODE="local"
elif [ "$1" = "--docker" ]; then
    BUILD_MODE="docker"
fi

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

if [ "$BUILD_MODE" = "local" ]; then
    # 仅构建前端，跳过后端编译
    echo "⏭️  跳过后端编译（--local 模式）"
    echo ""
    echo "✅ 前端构建完成！"
    echo ""
    echo "📁 构建产物："
    echo "  - 静态文件: web/dist/"
    echo ""
    echo "💡 下一步："
    echo "  1. 将整个项目上传到 Linux 服务器"
    echo "  2. 在服务器上运行: cargo build --release"
    echo "  3. 运行: ./target/release/calc24"
    exit 0
fi

if [ "$OS" = "Linux" ] || [ "$BUILD_MODE" = "auto" -a "$OS" = "Linux" ]; then
    # 在 Linux 上直接编译
    echo "检测到 Linux 系统，直接编译..."
    cargo build --release
    BINARY_PATH="target/release/calc24"
elif [ "$BUILD_MODE" = "docker" ] || [ "$BUILD_MODE" = "auto" ]; then
    # 使用 Docker 交叉编译
    echo "检测到非 Linux 系统，使用 Docker 交叉编译..."
    
    if ! command -v docker &> /dev/null; then
        echo "❌ 错误: 未找到 Docker"
        echo ""
        echo "请选择以下方案之一："
        echo "1. 安装 Docker Desktop: https://www.docker.com/products/docker-desktop"
        echo "2. 使用 --local 模式仅构建前端："
        echo "   ./build.sh --local"
        echo "   然后在 Linux 服务器上编译后端"
        exit 1
    fi
    
    echo "使用 Docker 构建 Linux 可执行文件..."
    echo "⏳ 首次运行需要下载 Docker 镜像，可能需要几分钟..."
    docker run --rm \
        -v "$(pwd)":/workspace \
        -w /workspace \
        rust:latest \
        bash -c "cargo build --release"
    
    BINARY_PATH="target/release/calc24"
fi

echo ""
echo "✅ 构建完成！"
echo ""
echo "📁 构建产物："
echo "  - 可执行文件: $BINARY_PATH"
echo "  - 静态文件: web/dist/"
echo ""
echo "📦 部署文件列表："
echo "  1. $BINARY_PATH (可执行文件)"
echo "  2. web/dist/ (整个目录)"
echo ""
echo "💡 使用 ./deploy.sh 部署到服务器"
