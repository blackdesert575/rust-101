# rust-101
rust-101 for one who want to learn Rust from scratch and practice more.

## to-do-list

* 調整自學行程/rust-101目錄結構

🎯 建議優先順序（若要先精熟一門語言）
若你未來想改善 Python 效能瓶頸 ➜ 先學 Rust

若你想開發小型工具/部署用 CLI ➜ 先學 Golang

若你想精通 DevOps / K8s Operator ➜ 先 Golang，後 Rust

# Rust 自學進度與實作規劃（針對 Python 效能優化）

本自學計畫針對具 Python 實作背景的開發者，透過 Rust 強化效能瓶頸，並逐步導入 CLI/系統層級的實作能力。

---

## 🎯 學習目標
- 精通 Rust 記憶體模型與系統程式設計能力
- 應用於現有 Python 專案（如：`sauce-man`, homelab script）效能重構
- 完成 Rust 基礎後，逐步導入 LeetCode 資料結構實作 + CLI 化

## ⏰ 自學時間限制
- 每週可投入時間：**3～5 天**
- 每日最多投入時間：**4 小時**

---

## 📘 Rust 自學進度（4 週規劃）

### ✅ Week 1：Ownership 完整練習
- 📖 學習：Chapter 4
  - [4.3] Fixing Ownership Errors
  - [4.4] The Slice Type
  - [4.5] Ownership Recap
- 🛠 練習：
  - 建立 `scripts/sandbox_ownership/`，重現錯誤情境與修正範例
  - 提取 `passwd-gen` 中可練習引用借用的邏輯片段

---

### ✅ Week 2：Struct / Enum / Pattern Matching
- 📖 學習：Chapter 5~6
  - Struct, Method
  - Enum, Pattern Matching
- 🛠 實作：
  - `poker/` 加入 Struct 與 Enum 表示邏輯
  - 練習 match 的 CLI 工具 → `scripts/pattern_game/`

---

### ✅ Week 3：Modules, Collections, Error Handling
- 📖 學習：Chapter 7~9
  - 模組系統、可見性
  - Vec / HashMap
  - `Result`, `unwrap`, 錯誤處理
- 🛠 實作：
  - `passwd-gen` 重構並加入錯誤處理邏輯
  - 撰寫測試、嘗試拆分 library 模組

---

### ✅ Week 4：Generics, Traits, Lifetimes
- 📖 學習：Chapter 10~11
  - 泛型、Trait Bounds
  - Lifetime 限制與應用
- 🛠 實作：
  - 泛型邏輯加入 `poker/`
  - 建立 `integration/sauce-man-rewrite/` 作為重構起點

---

## 🧠 Week 5 起延伸：LeetCode 實作 + CLI 化

每週安排：
| 類別 | 項目 | 說明 |
|------|------|------|
| LeetCode | 1 題 | 以 Rust 撰寫資料結構 / 演算法題 |
| 系統實作 | 2~4 天 | 將 LeetCode 題目 CLI 化工具（如：LRU Cache） |
| 進階延伸 | 選配 | 將 CLI 工具升級為 API 服務（FastAPI / actix） |

實作例子：
- LRU Cache → `scripts/lru-cli/`
- BST Traversal → `scripts/bst-cli/`

---

## 📁 rust-101 專案目錄建議調整
```
rust-101/
├── the_book/
│   ├── chapter04_ownership/
│   ├── chapter05_structs/
│   ├── chapter06_enum/
│   ├── chapter07_modules/
│   └── ...
├── tools/
│   ├── passwd-gen/
│   └── poker/
├── scripts/
│   ├── readme_gen/
│   ├── sandbox_ownership/
│   ├── pattern_game/
│   └── lru-cli/
├── integration/
│   └── sauce-man-rewrite/
└── online_judges/
    ├── exercism/
    ├── hackerrank/
    └── leetcode/
```

---

如需自動生成學習任務 README、建立 sandbox 模板、補充 LeetCode CLI 題庫，請聯絡 ChatGPT 😎

## Dir Layouts

```
.
├── online_judges
├── passwd-gen
├── poker
├── scripts
└── the_book

5 directories
```

## Docs/Guides/Tips/MISC/...etc

* [rust-lang.github.io/rustup](https://rust-lang.github.io/rustup/index.html)
* [github.com/plabayo/learn-rust-101](https://rust-lang.guide/intro/index.html)
* [Learn Rust](https://www.rust-lang.org/learn)
  * [the_book: The Rust Programming Language](https://rust-book.cs.brown.edu/)
    * [rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html)
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
* exercism
  * [github.com/exercism](https://github.com/exercism)
  * [exercism](https://exercism.org/)
* leetcode
* hackerrank
* codility

## Command Line App

* passwd-gen

```
./target/release/passwd-gen --use_upper --use_lower --use_numbers --no_symbols --length 16
```