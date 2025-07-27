# rust-101
rust-101 for one who want to learn Rust with AI(ChatGPI...etc)/any sources from scratch and practice more.

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

## 📘 Rust 自學進度

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

## Docs/Guides/Tips/MISC/...etc

* [Learn Rust](https://www.rust-lang.org/learn)
  * the_book: The Rust Programming Language
    * The Rust Programming Language: Experimental Edition
      * [github.com/cognitive-engineering-lab/rust-book](https://github.com/cognitive-engineering-lab/rust-book)
      * [github.com/cognitive-engineering-lab/rust-book/tree/main](https://github.com/cognitive-engineering-lab/rust-book/tree/main)
      * [rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html)
* Rust by Example
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
* [rust-lang.github.io/rustup](https://rust-lang.github.io/rustup/index.html)
* [github.com/plabayo/learn-rust-101](https://rust-lang.guide/intro/index.html)
* online coding
  * exercism
    * [github.com/exercism](https://github.com/exercism)
    * [exercism](https://exercism.org/)
  * leetcode
  * hackerrank
  * codility
* bastion-host
  * [github.com/warp-tech/warpgate](https://github.com/warp-tech/warpgate)
* Web
  * [docs.rs/reqwest/latest/reqwest/](https://docs.rs/reqwest/latest/reqwest/)
* [docs.rs/axum/latest/axum/](https://docs.rs/axum/latest/axum/)

## Command Line App

* passwd-gen

```
./target/release/passwd-gen --use_upper --use_lower --use_numbers --no_symbols --length 16
```