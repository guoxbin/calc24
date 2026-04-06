# 技术设计概览

## 前端组件

### PokerCards 组件
扇形展示 4 张扑克牌的 React 组件：

```typescript
interface PokerCardsProps {
    cards: string[]  // 4 张牌的值
}
```

**扇形布局参数：**
- 旋转角度：`[-18°, -6°, 6°, 18°]`
- 垂直偏移：`[15px, 5px, 5px, 15px]`
- 牌面尺寸：80x120px（桌面）/ 60x90px（移动端）

---

## 核心算法

### 24点求解算法

采用递归回溯算法，支持一元和二元运算符：

1. **一元运算符**（预处理阶段）：
   - `factorial` - 阶乘运算

2. **二元运算符**（递归求解）：
   - `+` - 加法
   - `-` - 减法
   - `*` - 乘法
   - `/` - 除法
   - `pow` - 乘方
   - `sqrt` - 开方（`a^(1/b)`）
   - `log` - 对数（`log_b(a)`）

### 算法流程

```
generate_initial_states()     # 应用一元运算生成初始状态
  ↓
solve_recursive()             # 递归求解
  - 基准：只剩1个数字，检查是否等于24
  - 递归：选2个数字进行二元运算，继续递归
```

## 数据结构

### GameNumber
```rust
struct GameNumber {
    value: f64,      // 数值
    expr: String,    // 表达式字符串
}
```

### 请求/响应结构

**CalculateRequest**
```rust
{
    numbers: String,      // 逗号分隔的数字，如 "A,2,3,9"
    range: String,        // "basic" 或 "poker"
    operators: Vec<String> // 允许的运算符列表
}
```

**CalculateResponse**
```rust
{
    solutions: Vec<String> // 解法列表
}
```

**GenerateRequest**
```rust
{
    range: String,        // "basic" 或 "poker"
    operators: Vec<String> // 允许的运算符列表
}
```

**GenerateResponse**
```rust
{
    problem: String       // 生成的题目，如 "A, 2, 3, 9"
}
```

## 关键设计决策

1. **表达式格式化**：使用 HTML 标签（`<sup>`, `<sub>`）支持数学符号显示
2. **精度控制**：使用 `epsilon = 1e-6` 进行浮点数比较
3. **限制条件**：阶乘限制 0-12，开方/对数限制输入范围
4. **生成算法**：随机生成+验证，最多100次重试
