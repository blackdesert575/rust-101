# rust-101
rust-101 for one who want to learn Rust with AI(ChatGPI...etc)/any sources from scratch and practice more.

## Quick Start

```shell
cd BRFNTP/ch01
cargo run

#compile with rustc
rustc main.rs

#with scripts
./scripts/rustc.sh BRFNTP/ch01/src/main.rs
./main
```

## to-do-list

* Adjust my self-study schedule
* Modify rust-101 dir layout
* Implementation plan
* Rust self-study progress
* Rust with any other languages(Python/Mojo/C/C++/Java/Golang/Perl/Javascript/Typescript)? (FFI/ABI/...etc)
* The book 
  * hashmap
    - [ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value)
  * string
    - [Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
    - [The Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
    - [Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
    - [doc.rust-lang.org/rust-by-example/std/str.html](https://doc.rust-lang.org/rust-by-example/std/str.html)
  * mod
    * [doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use)
    * [doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-nested-paths-to-clean-up-large-use-lists](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-nested-paths-to-clean-up-large-use-lists)

---

## Learning Goals
- Rust basic
- coding with any crates
- refactoring or enhancemnet exist projects: [sauce-man](https://github.com/blackdesert575/sauce-man), ansible, terraguant...etc

## Self learning timing
- hours per each week for learning: 10 hours
- hours per each day for learning: 0.5 ~ 2 hours

---

## Reading and coding with books/online-resources/...etc

### Rust by Example

* [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/index.html)

### Rustlings

* 00~02

```shell
# open 2 terminal window or using tools to make 2 or more terminal sessions

# terminal window 1
# Installation
cargo install rustlings
# Initialization
rustlings init
# Moving into new directory
cd rustlings
# Starting Rustlings
rustlings

# terminal window 2
# edit the *.rs file with your favorite coding editor(vim, neovim, vscode, nano...etc)
vim intro1.rs
```

---

## Docs/Guides/Tips/MISC/...etc

* Rust by Example
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
* Books
  * Beginning Rust: From Novice to Professional (BRFNTP)
  * [Learn Rust](https://www.rust-lang.org/learn)
    * [the_book: The Rust Programming Language](https://doc.rust-lang.org/book/)
      * The Rust Programming Language: Experimental Edition
        * [github.com/cognitive-engineering-lab/rust-book](https://github.com/cognitive-engineering-lab/rust-book)
    * Cargo
      * [doc.rust-lang.org/cargo/getting-started/first-steps.html](https://doc.rust-lang.org/cargo/getting-started/first-steps.html)      
* Rustlings
  * Small exercises to get you used to reading and writing Rust code - Recommended in parallel to reading the official Rust book 📚️
  * [rustlings.rust-lang.org](https://rustlings.rust-lang.org/)
* podcast
  * [rustacean-station.org](https://rustacean-station.org/)

### MISC

* qrcode
  * [crates.io/crates/qrcode](https://crates.io/crates/qrcode)
* bastion-host
  * warpgate
    * [github.com/warp-tech/warpgate](https://github.com/warp-tech/warpgate)
* Web
  * [docs.rs/reqwest/latest/reqwest/](https://docs.rs/reqwest/latest/reqwest/)
  * [docs.rs/axum/latest/axum/](https://docs.rs/axum/latest/axum/)
* online coding
  * exercism
    * [github.com/exercism](https://github.com/exercism)
    * [exercism](https://exercism.org/)
  * leetcode
  * hackerrank
  * codility