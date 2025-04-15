# rust-101
rust-101 for one who want to learn Rust from scratch and practice more.

## to-do-list

* 調整自學行程/rust-101目錄結構

🎯 建議優先順序（若要先精熟一門語言）
若你未來想改善 Python 效能瓶頸 ➜ 先學 Rust

若你想開發小型工具/部署用 CLI ➜ 先學 Golang

若你想精通 DevOps / K8s Operator ➜ 先 Golang，後 Rust

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