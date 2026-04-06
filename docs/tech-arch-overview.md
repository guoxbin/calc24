# 技术架构概览

## 技术栈

### 后端
| 技术 | 用途 |
|------|------|
| Rust | 系统编程语言 |
| actix-web | 高性能 Web 框架 |
| actix-cors | CORS 中间件 |
| actix-files | 静态文件服务 |
| tokio | 异步运行时 |
| serde | 序列化/反序列化 |
| rand | 随机数生成 |

### 前端
| 技术 | 用途 |
|------|------|
| React | UI 框架 |
| Vite | 构建工具和开发服务器 |
| Modern CSS | 渐变、动画、响应式设计 |

## 项目结构

```
calc24/
├── src/
│   └── main.rs          # Rust 后端入口
├── web/                 # React 前端项目
│   ├── src/
│   ├── dist/            # 生产构建输出
│   └── package.json
├── docs/                # 文档汇总
├── specs/               # 迭代文档
├── Cargo.toml           # Rust 配置
├── start.sh             # 启动脚本
└── deploy.sh            # 部署脚本
```

## 部署架构

- 后端服务监听 `0.0.0.0:3001`（生产模式）或 `127.0.0.1:3001`（开发模式）
- 前端开发服务器端口 5173
- 静态文件从 `web/dist` 目录服务
