#!/bin/bash

# 发布脚本 - 创建 Git 标签并推送到 GitHub 触发自动构建
#
# 使用方法:
#   ./deploy.sh <version>
#
# 示例:
#   ./deploy.sh 1.0.6
#   ./deploy.sh 1.1.0

set -e

if [ -z "$1" ]; then
    echo "❌ 错误: 请提供版本号"
    echo ""
    echo "使用方法:"
    echo "  ./deploy.sh <version>"
    echo ""
    echo "示例:"
    echo "  ./deploy.sh 1.0.6"
    echo "  ./deploy.sh 1.1.0"
    echo ""
    echo "💡 提示: 版本号会自动添加 'v' 前缀"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

echo "🚀 准备发布版本 ${TAG}..."
echo ""

# 1. 检查工作区是否干净
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  工作区有未提交的更改:"
    git status --short
    echo ""
    read -p "是否继续? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ 已取消发布"
        exit 1
    fi
    
    echo ""
    echo "� 提交所有更改..."
    git add .
    git commit -m "Release ${TAG}"
fi

# 2. 检查标签是否已存在
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "❌ 错误: 标签 ${TAG} 已存在"
    echo ""
    echo "� 提示:"
    echo "  - 查看所有标签: git tag"
    echo "  - 删除标签: git tag -d ${TAG}"
    echo "  - 删除远程标签: git push origin :refs/tags/${TAG}"
    exit 1
fi

# 3. 创建标签
echo "🏷️  创建标签 ${TAG}..."
git tag "$TAG"

# 4. 推送到 GitHub
echo ""
echo "� 推送代码和标签到 GitHub..."
git push origin master
git push origin "$TAG"

echo ""
echo "✅ 发布成功！"
echo ""
echo "📋 后续步骤:"
echo "  1. 访问 GitHub Actions 查看构建进度:"
echo "     https://github.com/guoxbin/calc24/actions"
echo ""
echo "  2. 构建完成后，在 Releases 页面下载:"
echo "     https://github.com/guoxbin/calc24/releases/tag/${TAG}"
echo ""
echo "  3. 部署到服务器:"
echo "     wget https://github.com/guoxbin/calc24/releases/download/${TAG}/calc24-linux-x86_64-musl.tar.gz"
echo "     tar -xzf calc24-linux-x86_64-musl.tar.gz"
echo "     cd calc24"
echo "     ./calc24"
