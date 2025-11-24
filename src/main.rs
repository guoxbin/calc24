use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::process::Command;

async fn index() -> HttpResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>算24点</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }
        
        .container {
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            padding: 60px 40px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            text-align: center;
            max-width: 850px;
            width: 100%;
            backdrop-filter: blur(10px);
        }
        
        h1 {
            font-size: 3.5em;
            color: #667eea;
            margin-bottom: 10px;
            font-weight: 700;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        
        .subtitle {
            font-size: 1.1em;
            color: #666;
            margin-bottom: 40px;
        }
        
        .range-options {
            margin-bottom: 25px;
            text-align: left;
        }
        
        .range-label {
            font-size: 1.1em;
            font-weight: 600;
            color: #667eea;
            margin-bottom: 15px;
            display: block;
        }
        
        .radio-group {
            display: flex;
            gap: 20px;
            justify-content: flex-start;
            flex-wrap: nowrap;
            align-items: center;
        }
        
        .radio-option {
            display: flex;
            align-items: center;
            cursor: pointer;
            padding: 10px 16px;
            border-radius: 10px;
            transition: all 0.3s ease;
            background: rgba(102, 126, 234, 0.05);
            white-space: nowrap;
        }
        
        .radio-option:hover {
            background: rgba(102, 126, 234, 0.15);
            transform: translateY(-2px);
        }
        
        .radio-option input[type="radio"] {
            width: 18px;
            height: 18px;
            margin-right: 8px;
            cursor: pointer;
            accent-color: #667eea;
            flex-shrink: 0;
        }
        
        .radio-option label {
            font-size: 0.95em;
            color: #333;
            cursor: pointer;
            user-select: none;
        }
        
        .input-section {
            margin-bottom: 30px;
        }
        
        .input-box {
            width: 100%;
            padding: 20px;
            font-size: 2em;
            text-align: center;
            border: 3px solid #667eea;
            border-radius: 15px;
            outline: none;
            transition: all 0.3s ease;
            letter-spacing: 0.5em;
            text-transform: uppercase;
            font-weight: 600;
        }
        
        .input-box:focus {
            border-color: #764ba2;
            box-shadow: 0 0 20px rgba(102, 126, 234, 0.3);
            transform: translateY(-2px);
        }
        
        .input-box::placeholder {
            color: #ccc;
            letter-spacing: normal;
        }
        
        .hint {
            margin-top: 10px;
            font-size: 0.9em;
            color: #888;
        }
        
        .calc-button {
            width: 100%;
            padding: 18px;
            font-size: 1.8em;
            font-weight: 700;
            color: white;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            border: none;
            border-radius: 15px;
            cursor: pointer;
            transition: all 0.3s ease;
            box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
        }
        
        .calc-button:hover {
            transform: translateY(-3px);
            box-shadow: 0 6px 25px rgba(102, 126, 234, 0.6);
        }
        
        .calc-button:active {
            transform: translateY(-1px);
        }
        
        .calc-button:disabled {
            opacity: 0.6;
            cursor: not-allowed;
            transform: none;
        }
        
        .result-section {
            margin-top: 40px;
            padding: 30px;
            background: rgba(102, 126, 234, 0.1);
            border-radius: 15px;
            min-height: 150px;
            display: none;
        }
        
        .result-section.show {
            display: block;
            animation: fadeIn 0.5s ease;
        }
        
        @keyframes fadeIn {
            from {
                opacity: 0;
                transform: translateY(10px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }
        
        .result-title {
            font-size: 1.3em;
            font-weight: 600;
            color: #667eea;
            margin-bottom: 20px;
        }
        
        .solution {
            font-size: 1.2em;
            color: #333;
            line-height: 1.8;
            text-align: left;
            padding: 15px;
            background: white;
            border-radius: 10px;
            margin-bottom: 10px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }
        
        .no-solution {
            font-size: 1.2em;
            color: #e74c3c;
            font-weight: 600;
        }
        
        .error {
            color: #e74c3c;
            font-size: 1em;
            margin-top: 10px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>算24点</h1>
        <p class="subtitle">输入4个数字，计算如何得到24</p>
        
        <div class="range-options">
            <span class="range-label">数字范围：</span>
            <div class="radio-group">
                <div class="radio-option">
                    <input 
                        type="radio" 
                        id="range-basic" 
                        name="range" 
                        value="basic" 
                        checked
                        onchange="updateHint()"
                    />
                    <label for="range-basic">A, 2, 3, 4, 5, 6, 7, 8, 9（A代表1）</label>
                </div>
                <div class="radio-option">
                    <input 
                        type="radio" 
                        id="range-poker" 
                        name="range" 
                        value="poker"
                        onchange="updateHint()"
                    />
                    <label for="range-poker">A, 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K（A, J, Q, K代表1, 11, 12, 13）</label>
                </div>
            </div>
        </div>
        
        <div class="range-options">
            <span class="range-label">计算符号：</span>
            <div class="radio-group" style="flex-wrap: wrap; gap: 15px;">
                <div class="radio-option">
                    <input type="checkbox" id="op-add" name="operators" value="+" checked>
                    <label for="op-add">+</label>
                </div>
                <div class="radio-option">
                    <input type="checkbox" id="op-sub" name="operators" value="-" checked>
                    <label for="op-sub">-</label>
                </div>
                <div class="radio-option">
                    <input type="checkbox" id="op-mul" name="operators" value="*" checked>
                    <label for="op-mul">*</label>
                </div>
                <div class="radio-option">
                    <input type="checkbox" id="op-div" name="operators" value="/" checked>
                    <label for="op-div">/</label>
                </div>
                <div class="radio-option">
                    <input type="checkbox" id="op-fact" name="operators" value="factorial">
                    <label for="op-fact">阶乘</label>
                </div>
                <div class="radio-option">
                    <input type="checkbox" id="op-pow" name="operators" value="pow">
                    <label for="op-pow">乘方</label>
                </div>
                <div class="radio-option">
                    <input type="checkbox" id="op-sqrt" name="operators" value="sqrt">
                    <label for="op-sqrt">开方</label>
                </div>
                <div class="radio-option">
                    <input type="checkbox" id="op-log" name="operators" value="log">
                    <label for="op-log">log</label>
                </div>
            </div>
        </div>
        
        <div class="input-section">
            <input 
                type="text" 
                id="numbers" 
                class="input-box" 
                placeholder="例如: A239"
                maxlength="4"
            />
            <p class="hint" id="hint-text">可输入 A, 2, 3, 4, 5, 6, 7, 8, 9（A代表1）</p>
        </div>
        
        <button class="calc-button" onclick="calculate()">算</button>
        
        <div id="result" class="result-section"></div>
    </div>
    
    <script>
        function updateHint() {
            const hintText = document.getElementById('hint-text');
            const isPoker = document.getElementById('range-poker').checked;
            
            if (isPoker) {
                hintText.textContent = '可输入 A, 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K（A/J/Q/K代表1/11/12/13）';
                document.getElementById('numbers').placeholder = '例如: AJQ2';
            } else {
                hintText.textContent = '可输入 A, 2, 3, 4, 5, 6, 7, 8, 9（A代表1）';
                document.getElementById('numbers').placeholder = '例如: A239';
            }
        }
        
        async function calculate() {
            const input = document.getElementById('numbers');
            const resultDiv = document.getElementById('result');
            const button = document.querySelector('.calc-button');
            const isPoker = document.getElementById('range-poker').checked;
            
            // 收集选中的运算符
            const selectedOps = Array.from(document.querySelectorAll('input[name="operators"]:checked'))
                .map(cb => cb.value);
                
            if (selectedOps.length === 0) {
                resultDiv.innerHTML = '<p class="error">请至少选择一个计算符号！</p>';
                resultDiv.classList.add('show');
                return;
            }
            
            const numbers = input.value.trim().toUpperCase();
            
            // 验证输入长度
            if (numbers.length !== 4 && numbers.length !== 5 && numbers.length !== 6) {
                resultDiv.innerHTML = '<p class="error">请输入4个数字！（10算作一个数字）</p>';
                resultDiv.classList.add('show');
                return;
            }
            
            // 根据选择的范围验证输入
            let validChars;
            let errorMsg;
            if (isPoker) {
                // 扑克牌范围：需要特殊处理10
                const pattern = /^(A|10|[2-9]|J|Q|K){4}$/;
                const tokens = numbers.match(/A|10|[2-9]|J|Q|K/g);
                if (!tokens || tokens.length !== 4) {
                    resultDiv.innerHTML = '<p class="error">只能输入 A, 2-9, 10, J, Q, K，且必须是4个数字！</p>';
                    resultDiv.classList.add('show');
                    return;
                }
            } else {
                // 基础范围
                validChars = /^[A2-9]{4}$/;
                if (!validChars.test(numbers)) {
                    resultDiv.innerHTML = '<p class="error">只能输入 A, 2-9！</p>';
                    resultDiv.classList.add('show');
                    return;
                }
            }
            
            // 禁用按钮
            button.disabled = true;
            button.textContent = '计算中...';
            
            try {
                const response = await fetch('/calculate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ 
                        numbers: numbers,
                        range: isPoker ? 'poker' : 'basic',
                        operators: selectedOps
                    })
                });
                
                const data = await response.json();
                
                if (data.solutions && data.solutions.length > 0) {
                    let html = '<div class="result-title">找到以下解法：</div>';
                    data.solutions.forEach((solution, index) => {
                        html += `<div class="solution">${index + 1}. ${solution}</div>`;
                    });
                    resultDiv.innerHTML = html;
                } else {
                    resultDiv.innerHTML = '<p class="no-solution">无解！这4个数字无法算出24。</p>';
                }
                
                resultDiv.classList.add('show');
            } catch (error) {
                resultDiv.innerHTML = '<p class="error">计算出错，请重试！</p>';
                resultDiv.classList.add('show');
            } finally {
                button.disabled = false;
                button.textContent = '算';
            }
        }
        
        // 回车键触发计算
        document.getElementById('numbers').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                calculate();
            }
        });
    </script>
</body>
</html>
    "#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[derive(Deserialize)]
struct CalculateRequest {
    numbers: String,
    range: String,
    operators: Vec<String>,
}

#[derive(Serialize)]
struct CalculateResponse {
    solutions: Vec<String>,
}

#[derive(Deserialize)]
struct GenerateRequest {
    range: String,
    operators: Vec<String>,
}

#[derive(Serialize)]
struct GenerateResponse {
    problem: String,
}

async fn calculate(req: web::Json<CalculateRequest>) -> HttpResponse {
    let numbers = &req.numbers;
    let range = &req.range;
    let operators = &req.operators;

    // 将输入转换为数字数组
    let mut nums: Vec<f64> = if range == "poker" {
        // 扑克牌范围：需要特殊处理 10, J, Q, K
        parse_poker_numbers(numbers)
    } else {
        // 基础范围：A, 2-9
        numbers
            .chars()
            .map(|c| match c {
                'A' => 1.0,
                '2'..='9' => c.to_digit(10).unwrap() as f64,
                _ => 0.0,
            })
            .collect()
    };

    if nums.len() != 4 {
        return HttpResponse::BadRequest().json(CalculateResponse { solutions: vec![] });
    }

    let solutions = solve_24(&mut nums, operators);

    HttpResponse::Ok().json(CalculateResponse { solutions })
}

async fn generate_problem(req: web::Json<GenerateRequest>) -> HttpResponse {
    let range = &req.range;
    let operators = &req.operators;
    let mut rng = rand::thread_rng();

    // 限制重试次数，防止死循环（虽然不太可能）
    for _ in 0..100 {
        let mut nums = Vec::new();
        let mut display_chars = Vec::new();

        for _ in 0..4 {
            let val = if range == "poker" {
                // 1-13
                use rand::Rng;
                rng.gen_range(1..=13) as f64
            } else {
                // 1-9
                use rand::Rng;
                rng.gen_range(1..=9) as f64
            };
            nums.push(val);

            // 转换为显示字符
            let s = match val as i32 {
                1 => "A".to_string(),
                10 => "10".to_string(),
                11 => "J".to_string(),
                12 => "Q".to_string(),
                13 => "K".to_string(),
                n => n.to_string(),
            };
            display_chars.push(s);
        }

        // 验证是否有解
        let solutions = solve_24(&mut nums.clone(), operators);
        if !solutions.is_empty() {
            return HttpResponse::Ok().json(GenerateResponse {
                problem: display_chars.join(""),
            });
        }
    }

    // 如果实在找不到（极低概率），返回一个默认的
    HttpResponse::Ok().json(GenerateResponse {
        problem: "AAAA".to_string(),
    })
}

fn parse_poker_numbers(input: &str) -> Vec<f64> {
    let mut nums = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let num = match c {
            'A' => 1.0,
            '1' => {
                // 检查是否是 10
                if chars.peek() == Some(&'0') {
                    chars.next(); // 消费 '0'
                    10.0
                } else {
                    1.0
                }
            }
            '2'..='9' => c.to_digit(10).unwrap() as f64,
            'J' => 11.0,
            'Q' => 12.0,
            'K' => 13.0,
            _ => continue,
        };
        nums.push(num);
    }

    nums
}

#[derive(Clone)]
struct GameNumber {
    value: f64,
    expr: String,
}

impl GameNumber {
    fn new(val: f64) -> Self {
        Self {
            value: val,
            expr: format_num(val),
        }
    }
}

fn solve_24(nums: &mut Vec<f64>, allowed_ops: &[String]) -> Vec<String> {
    let mut solutions = Vec::new();
    // let mut used = vec![false; 4]; // No longer directly used in this refactored version
    // let mut current = Vec::new(); // No longer directly used in this refactored version

    // 预处理二元运算符
    let binary_ops: Vec<String> = allowed_ops
        .iter()
        .filter(|op| ["+", "-", "*", "/", "pow", "sqrt"].contains(&op.as_str()))
        .cloned()
        .collect();

    // 预处理一元运算符
    let unary_ops: Vec<String> = allowed_ops
        .iter()
        .filter(|op| ["factorial", "log"].contains(&op.as_str()))
        .cloned()
        .collect();

    // 1. 生成所有可能的初始状态（对初始数字应用一元运算符）
    let initial_states = generate_initial_states(nums, &unary_ops);

    // 2. 对每个初始状态进行递归求解
    for state in initial_states {
        solve_recursive(state, &mut solutions, &binary_ops, &unary_ops);
    }

    // 去重
    solutions.sort();
    solutions.dedup();

    solutions
}

// 生成初始状态：每个数字都可以选择保持原样，或者应用一元运算符
fn generate_initial_states(nums: &Vec<f64>, unary_ops: &Vec<String>) -> Vec<Vec<GameNumber>> {
    let mut states = Vec::new();
    let mut current_state = Vec::new();
    generate_states_recursive(nums, 0, &mut current_state, &mut states, unary_ops);
    states
}

fn generate_states_recursive(
    nums: &Vec<f64>,
    index: usize,
    current_state: &mut Vec<GameNumber>,
    states: &mut Vec<Vec<GameNumber>>,
    unary_ops: &Vec<String>,
) {
    if index == nums.len() {
        states.push(current_state.clone());
        return;
    }

    let val = nums[index];
    let base_num = GameNumber::new(val);

    // 选项1：保持原样
    current_state.push(base_num.clone());
    generate_states_recursive(nums, index + 1, current_state, states, unary_ops);
    current_state.pop();

    // 选项2：应用阶乘
    if unary_ops.contains(&"factorial".to_string()) {
        if let Some(fact_val) = factorial(val) {
            current_state.push(GameNumber {
                value: fact_val,
                expr: format!("({})!", base_num.expr),
            });
            generate_states_recursive(nums, index + 1, current_state, states, unary_ops);
            current_state.pop();
        }
    }

    // 选项3：应用 log
    if unary_ops.contains(&"log".to_string()) {
        if val > 0.0 && val != 1.0 {
            let log_val = val.log10();
            current_state.push(GameNumber {
                value: log_val,
                expr: format!("log({})", base_num.expr),
            });
            generate_states_recursive(nums, index + 1, current_state, states, unary_ops);
            current_state.pop();
        }
    }
}

fn solve_recursive(
    nums: Vec<GameNumber>,
    solutions: &mut Vec<String>,
    binary_ops: &Vec<String>,
    unary_ops: &Vec<String>,
) {
    let epsilon = 1e-6;

    // 基准情况：只剩一个数字
    if nums.len() == 1 {
        let n = &nums[0];

        // 检查是否等于 24
        if (n.value - 24.0).abs() < epsilon {
            solutions.push(format!("{} = 24", n.expr));
        }

        // 尝试对最终结果应用一元运算
        // 阶乘
        if unary_ops.contains(&"factorial".to_string()) {
            if let Some(fact_val) = factorial(n.value) {
                if (fact_val - 24.0).abs() < epsilon {
                    solutions.push(format!("({})! = 24", n.expr));
                }
            }
        }
        // Log (虽然 log(x)=24 意味着 x=10^24，不太可能，但为了完整性)
        if unary_ops.contains(&"log".to_string()) {
            if n.value > 0.0 && n.value != 1.0 {
                let log_val = n.value.log10();
                if (log_val - 24.0).abs() < epsilon {
                    solutions.push(format!("log({}) = 24", n.expr));
                }
            }
        }

        return;
    }

    // 递归步骤：选取两个数字进行运算
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }

            // 提取 nums[i] 和 nums[j]
            // 为了方便处理剩余列表，我们先构建剩余列表
            let mut next_nums = Vec::with_capacity(nums.len() - 1);
            for k in 0..nums.len() {
                if k != i && k != j {
                    next_nums.push(nums[k].clone());
                }
            }

            let a = &nums[i];
            let b = &nums[j];

            // 尝试所有二元运算符
            for op in binary_ops {
                // 计算 a op b
                if let Some(res_val) = eval_binary(a.value, b.value, op) {
                    let res_expr = fmt_op(&a.expr, &b.expr, op);
                    let res_num = GameNumber {
                        value: res_val,
                        expr: res_expr,
                    };

                    // 路径1：将结果直接加入列表递归
                    let mut path1_nums = next_nums.clone();
                    path1_nums.push(res_num.clone());
                    solve_recursive(path1_nums, solutions, binary_ops, unary_ops);

                    // 路径2：对结果应用一元运算后加入列表递归
                    // 阶乘
                    if unary_ops.contains(&"factorial".to_string()) {
                        if let Some(fact_val) = factorial(res_val) {
                            let fact_num = GameNumber {
                                value: fact_val,
                                expr: format!("({})!", res_num.expr),
                            };
                            let mut path2_nums = next_nums.clone();
                            path2_nums.push(fact_num);
                            solve_recursive(path2_nums, solutions, binary_ops, unary_ops);
                        }
                    }
                    // Log
                    if unary_ops.contains(&"log".to_string()) {
                        if res_val > 0.0 && res_val != 1.0 {
                            let log_val = res_val.log10();
                            let log_num = GameNumber {
                                value: log_val,
                                expr: format!("log({})", res_num.expr),
                            };
                            let mut path3_nums = next_nums.clone();
                            path3_nums.push(log_num);
                            solve_recursive(path3_nums, solutions, binary_ops, unary_ops);
                        }
                    }
                }
            }
        }
    }
}

// 辅助函数：格式化二元运算表达式
fn fmt_op(lhs: &str, rhs: &str, op: &str) -> String {
    match op {
        "pow" => format!("{}^{}", lhs, rhs),
        "sqrt" => format!("{}√{}", rhs, lhs), // b√a (b次根号a)
        _ => format!("({} {} {})", lhs, op, rhs),
    }
}

fn eval_binary(a: f64, b: f64, op: &str) -> Option<f64> {
    match op {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => {
            if b.abs() < 1e-6 {
                None
            } else {
                Some(a / b)
            }
        }
        "pow" => {
            let res = a.powf(b);
            if res.is_finite() && res.abs() < 10000.0 {
                Some(res)
            } else {
                None
            }
        }
        "sqrt" => {
            // a 是被开方数，b 是开方次数
            // 结果是 a^(1/b)
            if b.abs() < 1e-6 {
                return None;
            } // 开方次数不能为0
            if a < 0.0 {
                return None;
            } // 简单起见，暂不支持负数开方

            let res = a.powf(1.0 / b);
            if res.is_finite() && res.abs() < 10000.0 {
                Some(res)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn factorial(n: f64) -> Option<f64> {
    let int_n = n.round() as i32;
    if int_n < 0 || int_n > 12 || (n - int_n as f64).abs() > 1e-6 {
        return None;
    }

    let mut result = 1.0;
    for i in 2..=int_n {
        result *= i as f64;
    }
    Some(result)
}

fn format_num(n: f64) -> String {
    let int_val = n.round() as i32;
    match int_val {
        1 => "A".to_string(),
        10 => "10".to_string(),
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        _ => format!("{}", int_val),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8088;
    let url = format!("http://localhost:{}", port);

    println!("🚀 启动算24点服务器...");
    println!("📡 服务地址: {}", url);
    println!("按 Ctrl+C 停止服务器");

    // 在新线程中打开浏览器
    let url_clone = url.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("🌐 正在打开浏览器...");
        let _ = open_browser(&url_clone);
    });

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/calculate", web::post().to(calculate))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn open_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(url).spawn()?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd").args(&["/C", "start", url]).spawn()?;
    }

    Ok(())
}
