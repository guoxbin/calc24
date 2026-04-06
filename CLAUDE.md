# CLAUDE.md

## 第一性原则

从需求和问题本质出发，不从惯例或模板出发。

1. 不要假设我清楚自己想要什么。动机或目标不清晰就停下来讨论。
2. 目标清晰但路径不是最短的，直接告诉我并建议更好的办法。
3. 遇到问题追根因，不打补丁。每个决策都要能回答"为什么"。
4. 输出说重点，砍掉一切不改变决策的信息。

## Documentation Conventions

### specs/

历次迭代的文档，每次迭代一个文件夹，包含：

- `1-requirements.md` - 需求文档
- `2-research.md` - 调研文档（简单变更可省略）
- `3-tech-design.md` - 技术设计文档
- `4-tasks.md` - 任务清单
- `5-review.md` - 迭代复盘（迭代完成后编写）

### docs/

项目汇总文档，根据每次迭代整理，始终反映最新状态：

- `requirements-overview.md` - 需求概览
- `tech-arch-overview.md` - 技术架构概览
- `tech-design-overview.md` - 技术设计概览
- `tech-api-overview.yaml` - API 接口概览（OAS 3.1）
- `tech-memory-overview.md` - 技术记忆与知识
- `tech-rule-overview.md` - 技术规范
