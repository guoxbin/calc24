#!/bin/bash

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 清理函数
cleanup() {
    echo -e "\n${RED}正在停止服务...${NC}"
    if [ ! -z "$BACKEND_PID" ]; then
        kill $BACKEND_PID 2>/dev/null
        echo -e "${GREEN}后端服务已停止${NC}"
    fi
    if [ ! -z "$FRONTEND_PID" ]; then
        kill $FRONTEND_PID 2>/dev/null
        echo -e "${GREEN}前端服务已停止${NC}"
    fi
    exit 0
}

# 捕获退出信号
trap cleanup SIGINT SIGTERM

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}    启动算24点应用${NC}"
echo -e "${BLUE}========================================${NC}"

# 1. 启动后端
echo -e "\n${GREEN}[1/3] 启动后端服务...${NC}"
cargo run &
BACKEND_PID=$!

# 2. 等待后端就绪
echo -e "${GREEN}[2/3] 等待后端就绪...${NC}"
for i in {1..30}; do
    if curl -s http://localhost:3001 >/dev/null 2>&1; then
        echo -e "${GREEN}✓ 后端服务已就绪 (PID: $BACKEND_PID)${NC}"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "${RED}✗ 后端启动超时${NC}"
        cleanup
    fi
    sleep 1
done

# 3. 启动前端
echo -e "${GREEN}[3/3] 启动前端服务...${NC}"
cd web
npm run dev &
FRONTEND_PID=$!

echo -e "\n${BLUE}========================================${NC}"
echo -e "${GREEN}✓ 应用启动成功！${NC}"
echo -e "${BLUE}========================================${NC}"
echo -e "后端地址: ${GREEN}http://localhost:3001${NC}"
echo -e "前端地址: ${GREEN}http://localhost:5173${NC}"
echo -e "\n按 ${RED}Ctrl+C${NC} 停止所有服务"
echo -e "${BLUE}========================================${NC}\n"

# 等待前端进程
wait $FRONTEND_PID
