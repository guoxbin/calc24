# 技术设计：扇形扑克牌背景装饰

## 方案概述
使用 CSS + SVG 实现一个写实风格的扇形扑克牌扇面。

## 实现方案

### 方案选择
采用 **SVG + CSS** 实现：
- SVG 绘制每张扑克牌的精细图案（花色、数字）
- CSS transform 实现扇形旋转和定位
- 多层结构实现立体感和层叠效果

### 扇形结构

```
                    [K♠]
                  [Q♠]
                [J♠]
              [10♠]
            [9♦]
          [8♦]
        [A♥]  ← 中心牌，几乎垂直
          [2♣]
            [3♣]
              [4♣]
                [5♣]
                  [6♣]
                    [7♣]
```

- 约 13-15 张牌
- 扇形角度：约 70°（-35° 到 +35°）
- 旋转中心点：统一在扇形底部（画面外）

### SVG 扑克牌设计

每张牌包含：
- 外框：圆角矩形，白色填充，细边框
- 牌面：
  - 左上角：数字 + 小花色符号
  - 中央：大花色符号或人物图案（简化为符号）
  - 右下角：倒置的数字 + 小花色符号（旋转 180°）
- 阴影：底部投影增加立体感

### CSS 定位

```css
.fan-container {
  position: fixed;
  bottom: -60px;  /* 扇形中心在画面外 */
  left: 50%;
  transform: translateX(-50%);
  width: 600px;
  height: 400px;
  pointer-events: none;
  opacity: 0.18;
}

.poker-card {
  position: absolute;
  width: 70px;
  height: 100px;
  transform-origin: bottom center;
  transform: rotate(var(--angle)) translateY(var(--offset));
}
```

### 牌面数据

```javascript
const cards = [
  { rank: 'K', suit: '♠', angle: -35 },
  { rank: 'Q', suit: '♠', angle: -30 },
  { rank: 'J', suit: '♠', angle: -25 },
  { rank: '10', suit: '♥', angle: -20 },
  { rank: '9', suit: '♥', angle: -15 },
  { rank: '8', suit: '♥', angle: -10 },
  { rank: 'A', suit: '♦', angle: -5 },
  { rank: '2', suit: '♦', angle: 0 },    // 中心
  { rank: '3', suit: '♦', angle: 5 },
  { rank: '4', suit: '♣', angle: 10 },
  { rank: '5', suit: '♣', angle: 15 },
  { rank: '6', suit: '♣', angle: 20 },
  { rank: '7', suit: '♣', angle: 25 },
  { rank: '8', suit: '♣', angle: 30 },
  { rank: '9', suit: '♣', angle: 35 },
];
```

## 文件变更

| 文件 | 说明 |
|------|------|
| `web/src/components/FanCards.tsx` | 扇形牌组组件 |
| `web/src/components/FanCards.css` | 组件样式 |
| `web/src/App.tsx` | 引入组件 |

## 响应式

- 桌面端：完整显示，600px 宽
- 平板：缩放至 80%
- 移动端：缩放至 60%，或改为角落小扇形

## 技术要点

1. **transform-origin**: 所有牌统一以底部中心为旋转原点
2. **z-index**: 中间牌的 z-index 更高，模拟层叠
3. **SVG viewBox**: 每张牌独立 SVG，便于复用
4. **颜色**: 红桃♥、方块♦ 用红色；黑桃♠、梅花♣ 用黑色

## 待决策

- [ ] 扇形位置：底部中央 / 左下角 / 右下角？
- [ ] 是否添加轻微模糊效果（filter: blur(1px)）？
- [ ] 是否添加 hover 微动效？（建议不加，保持纯背景）
