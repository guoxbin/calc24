use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    let _range = &req.range;
    let operators = &req.operators;

    // 将输入转换为数字数组
    // 输入现在是逗号分隔的字符串
    let tokens: Vec<&str> = numbers.split(',').map(|s| s.trim()).collect();

    if tokens.len() != 4 {
        return HttpResponse::BadRequest().json(CalculateResponse { solutions: vec![] });
    }

    let mut nums: Vec<f64> = Vec::new();

    for token in tokens {
        let val = match token {
            "A" => 1.0,
            "J" => 11.0,
            "Q" => 12.0,
            "K" => 13.0,
            "10" => 10.0,
            t => {
                if let Ok(v) = t.parse::<f64>() {
                    v
                } else {
                    return HttpResponse::BadRequest()
                        .json(CalculateResponse { solutions: vec![] });
                }
            }
        };
        nums.push(val);
    }

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
                problem: display_chars.join(", "),
            });
        }
    }

    // 如果实在找不到（极低概率），返回一个默认的
    HttpResponse::Ok().json(GenerateResponse {
        problem: "A, A, A, A".to_string(),
    })
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

    // 预处理二元运算符
    let binary_ops: Vec<String> = allowed_ops
        .iter()
        .filter(|op| ["+", "-", "*", "/", "pow", "sqrt", "log"].contains(&op.as_str()))
        .cloned()
        .collect();

    // 预处理一元运算符
    let unary_ops: Vec<String> = allowed_ops
        .iter()
        .filter(|op| ["factorial"].contains(&op.as_str()))
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
                }
            }
        }
    }
}

// 辅助函数：格式化二元运算表达式
fn fmt_op(lhs: &str, rhs: &str, op: &str) -> String {
    match op {
        "pow" => format!("{}^{}", lhs, rhs),
        "sqrt" => format!("<sup>{}</sup>√{}", rhs, lhs),
        "log" => format!("log<sub>{}</sub>{}", rhs, lhs),
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
        "log" => {
            // log_b(a) = log(a) / log(b)
            // a 是真数，b 是底数
            if a <= 0.0 || b <= 0.0 || b == 1.0 {
                return None;
            }
            let res = a.log10() / b.log10();
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
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);

    // 检查是否存在静态文件目录
    let static_dir = PathBuf::from("web/dist");
    let serve_static = static_dir.exists();

    if serve_static {
        println!("🚀 启动算24点服务器（生产模式）...");
        println!(
            "📁 静态文件目录: {:?}",
            static_dir.canonicalize().unwrap_or(static_dir.clone())
        );
    } else {
        println!("🚀 启动算24点后端服务器（开发模式）...");
        println!("⚠️  未找到静态文件目录 web/dist，仅提供 API 服务");
    }

    let bind_addr = if serve_static {
        "0.0.0.0" // 生产模式：监听所有网络接口
    } else {
        "127.0.0.1" // 开发模式：仅监听本地
    };

    let url = format!("http://{}:{}", bind_addr, port);
    println!("📡 服务地址: {}", url);
    println!("按 Ctrl+C 停止服务器");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        let mut app = App::new().wrap(cors).service(
            web::scope("/api")
                .route("/calculate", web::post().to(calculate))
                .route("/generate", web::post().to(generate_problem)),
        );

        // 如果存在静态文件目录，则服务静态文件
        if serve_static {
            app = app
                .service(fs::Files::new("/assets", "web/dist/assets").show_files_listing())
                .service(fs::Files::new("/", "web/dist").index_file("index.html"));
        }

        app
    })
    .bind((bind_addr, port))?
    .run()
    .await
}
