# 技术规范

## 代码规范

### Rust
- 使用标准格式化工具 `rustfmt`
- 错误处理使用标准库 `Option<T>` 和 `Result<T, E>`
- 结构体字段使用蛇形命名法
- 常量使用大写蛇形命名法

### API 设计
- RESTful API 风格
- 端点统一使用 `/api/` 前缀
- POST 请求体使用 JSON 格式
- 响应统一使用 JSON 格式

## 项目约定

### 目录结构
- `src/` - Rust 源代码
- `web/` - React 前端项目
- `web/dist/` - 前端生产构建输出
- `docs/` - 项目汇总文档
- `specs/` - 迭代文档

### 开发流程
- 使用 `cargo run` 启动后端开发服务器
- 使用 `npm run dev` (在 web/ 目录) 启动前端开发服务器
- 使用 `./start.sh` 一键启动前后端

## 文档规范

- 迭代文档使用 Markdown 格式
- API 文档使用 OpenAPI 3.1 (YAML)
- 保持文档与代码同步更新
