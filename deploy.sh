#!/bin/bash

# 部署脚本 - 将构建产物部署到 Linux 服务器

# 使用方法:
# ./deploy.sh user@server:/path/to/deploy

set -e

if [ -z "$1" ]; then
    echo "❌ 错误: 请提供部署目标"
    echo ""
    echo "使用方法:"
    echo "  ./deploy.sh user@server:/path/to/deploy"
    echo ""
    echo "示例:"
    echo "  ./deploy.sh root@192.168.1.100:/opt/calc24"
    echo "  ./deploy.sh ubuntu@example.com:/home/ubuntu/calc24"
    exit 1
fi

DEPLOY_TARGET=$1

# 查找可执行文件
if [ -f "target/release/calc24" ]; then
    BINARY_PATH="target/release/calc24"
elif [ -f "target/x86_64-unknown-linux-musl/release/calc24" ]; then
    BINARY_PATH="target/x86_64-unknown-linux-musl/release/calc24"
else
    echo "❌ 错误: 未找到可执行文件"
    echo "请先运行 ./build.sh 构建项目"
    exit 1
fi

if [ ! -d "web/dist" ]; then
    echo "❌ 错误: 未找到静态文件目录"
    echo "请先运行 ./build.sh 构建项目"
    exit 1
fi

echo "🚀 开始部署到 $DEPLOY_TARGET..."
echo "📦 可执行文件: $BINARY_PATH"

# 提取服务器地址和路径
SERVER=$(echo $DEPLOY_TARGET | cut -d: -f1)
REMOTE_PATH=$(echo $DEPLOY_TARGET | cut -d: -f2)

# 1. 创建远程目录
echo ""
echo "📁 创建远程目录..."
ssh $SERVER "mkdir -p $REMOTE_PATH/web"

# 2. 复制可执行文件
echo ""
echo "📤 上传可执行文件..."
scp $BINARY_PATH $SERVER:$REMOTE_PATH/calc24

# 3. 复制静态文件
echo ""
echo "📤 上传静态文件..."
scp -r web/dist $SERVER:$REMOTE_PATH/web/

# 4. 复制 systemd 服务文件（可选）
if [ -f "calc24.service" ]; then
    echo ""
    echo "📤 上传 systemd 服务文件..."
    scp calc24.service $SERVER:$REMOTE_PATH/
fi

# 5. 设置可执行权限
echo ""
echo "🔧 设置可执行权限..."
ssh $SERVER "chmod +x $REMOTE_PATH/calc24"

echo ""
echo "✅ 部署完成！"
echo ""
echo "📋 在服务器上运行:"
echo "  cd $REMOTE_PATH"
echo "  ./calc24"
echo ""
echo "💡 提示:"
echo "  - 默认端口: 3001"
echo "  - 自定义端口: PORT=8080 ./calc24"
echo "  - 后台运行: nohup ./calc24 > calc24.log 2>&1 &"
echo ""
echo "🔧 设置 systemd 服务（推荐）:"
echo "  sudo cp $REMOTE_PATH/calc24.service /etc/systemd/system/"
echo "  sudo nano /etc/systemd/system/calc24.service  # 修改路径和用户"
echo "  sudo systemctl daemon-reload"
echo "  sudo systemctl start calc24"
echo "  sudo systemctl enable calc24"
