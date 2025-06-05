use rand::Rng;
use std::env;

const DEFAULT_LENGTH: usize = 16;
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?/";

fn generate_password(length: usize, use_upper: bool, use_lower: bool, use_numbers: bool, use_symbols: bool) -> String {
    let mut charset = String::new();

    if use_upper { charset.push_str(UPPERCASE); }
    if use_lower { charset.push_str(LOWERCASE); }
    if use_numbers { charset.push_str(NUMBERS); }
    if use_symbols { charset.push_str(SYMBOLS); }

    if charset.is_empty() {
        eprintln!("❌ Error: 至少要選擇一種字元類型！");
        std::process::exit(1);
    }

    let mut rng = rand::rng(); // ✅ 使用 `rand::rng()` 取代 `StdRng::from_entropy()`
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len()); // ✅ `gen_range()` -> `random_range()`
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    println!("🔑 產生的密碼長度: {}", password.len());
    password
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut length = DEFAULT_LENGTH;
    let mut use_upper = true;
    let mut use_lower = true;
    let mut use_numbers = true;
    let mut use_symbols = true;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--use_upper" => use_upper = true,
            "--no_upper" => use_upper = false,
            "--use_lower" => use_lower = true,
            "--no_lower" => use_lower = false,
            "--use_numbers" => use_numbers = true,
            "--no_numbers" => use_numbers = false,
            "--use_symbols" => use_symbols = true,
            "--no_symbols" => use_symbols = false,
            "--length" => {
                if i + 1 < args.len() {
                    if let Ok(l) = args[i + 1].parse::<usize>() {
                        length = l;
                        i += 1;
                    } else {
                        eprintln!("❌ Error: `--length` 參數後面必須是一個有效的數字！");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("❌ Error: `--length` 參數缺少數值！");
                    std::process::exit(1);
                }
            }
            "--help" => {
                println!("🔹 使用方式: passwd-gen [選項]");
                println!("🔹 預設: 16 字元密碼，包含大小寫、數字、符號");
                println!("選項:");
                println!("  --use_upper      ✅ 啟用大寫英文字母 (預設啟用)");
                println!("  --no_upper       ❌ 停用大寫英文字母");
                println!("  --use_lower      ✅ 啟用小寫英文字母 (預設啟用)");
                println!("  --no_lower       ❌ 停用小寫英文字母");
                println!("  --use_numbers    ✅ 啟用數字 (預設啟用)");
                println!("  --no_numbers     ❌ 停用數字");
                println!("  --use_symbols    ✅ 啟用特殊符號 (預設啟用)");
                println!("  --no_symbols     ❌ 停用特殊符號");
                println!("  --length <數字>  📏 設定密碼長度（預設 16）");
                println!("  --help           ℹ️  顯示幫助資訊");
                println!("\n範例:");
                println!("  passwd-gen --use_upper --use_lower --use_numbers --length 12");
                println!("  passwd-gen --no_symbols --length 20");
                return;
            }
            _ => {
                eprintln!("⚠️  警告: 未知參數 `{}`，請使用 `--help` 查看可用選項！", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let password = generate_password(length, use_upper, use_lower, use_numbers, use_symbols);
    println!("✅ 產生的密碼: {}", password);
}