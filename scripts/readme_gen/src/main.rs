use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct StatusMap(HashMap<String, HashMap<String, String>>);

fn kebab_to_title(s: &str) -> String {
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn generate_readme(dir: &Path, difficulty: &str, status_map: &HashMap<String, HashMap<String, String>>) -> std::io::Result<()> {
    let entries = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .collect::<Vec<_>>();

    let output_path = dir.join("README.md");
    let mut file = BufWriter::new(File::create(output_path)?);

    writeln!(file, "# Problem Solving - {}", difficulty)?;
    writeln!(
        file,
        "\nRust solutions for HackerRank's Problem Solving challenges - **{}** difficulty.",
        difficulty
    )?;

    writeln!(file, "\n## 📘 Problems")?;
    writeln!(file, "\n| Problem | Description | Link | Status |")?;
    writeln!(file, "|--------|-------------|------|--------|")?;

    for entry in entries {
        let folder_name = entry.file_name().to_string_lossy().to_string();
        let display_name = kebab_to_title(&folder_name);
        let link = format!(
            "https://www.hackerrank.com/challenges/{}/problem",
            folder_name
        );

        let status = status_map
            .get(difficulty)
            .and_then(|m| m.get(&folder_name))
            .map(|s| match s.as_str() {
                "Done" => "✅ Done",
                "In Progress" => "⏳ In Progress",
                _ => "⬜ To Do",
            })
            .unwrap_or("⬜ To Do");

        writeln!(
            file,
            "| `{}` | {} | [🔗]({}) | {} |",
            folder_name, display_name, link, status
        )?;
    }

    writeln!(
        file,
        "\n> ✅ Done ｜ ⏳ In Progress ｜ ⬜ To Do"
    )?;

    writeln!(
        file,
        "\n## 🛠 Running a Problem\n\nTo run a specific problem:\n\n```bash\ncd <problem-folder>\ncargo run\n```\n\nTo test (if test cases are included):\n\n```bash\ncargo test\n```"
    )?;

    writeln!(
        file,
        "\n---\n\nFeel free to PR suggestions or improvements. Happy hacking! 🦀"
    )?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <Problem Solving 路徑>", args[0]);
        std::process::exit(1);
    }

    let root = PathBuf::from(&args[1]);

    if !root.exists() || !root.is_dir() {
        eprintln!("❌ 錯誤：指定路徑不是有效資料夾");
        std::process::exit(1);
    }

    // 嘗試載入 status.json
    let status_path = root.join("status.json");
    let status_data: StatusMap = if status_path.exists() {
        let data = fs::read_to_string(status_path).expect("讀取 status.json 失敗");
        serde_json::from_str(&data).expect("解析 status.json 錯誤")
    } else {
        println!("⚠️ 找不到 status.json，預設所有狀態為 ⬜ To Do");
        StatusMap(HashMap::new())
    };

    let status_map = &status_data.0;

    // 掃描每個難度層級
    let levels = fs::read_dir(&root)
        .expect("無法讀取主目錄")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir() && e.file_name().to_string_lossy() != "target");

    for level in levels {
        let path = level.path();
        let difficulty = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        match generate_readme(&path, &difficulty, status_map) {
            Ok(_) => println!("✅ 已產生：{}/README.md", difficulty),
            Err(e) => eprintln!("❌ 錯誤處理 {}: {}", difficulty, e),
        }
    }
}