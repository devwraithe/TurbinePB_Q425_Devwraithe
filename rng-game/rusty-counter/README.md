# 🔢 rusty_counter

A simple interactive **counter app** built in **Rust**. This project was built as one of Turbin3 Pre-Builders Cohort Challenges.

---

## 🎯 Overview

The counter lets you **add** or **remove** counts based on user input.  
You can choose from three simple options:

1. Add 1 to the counter  
2. Remove 1 from the counter  
3. Quit the counter  

---

## ⚙️ Installation

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

Clone this repository:

```bash
git clone https://github.com/devwraithe/rusty_counter.git
cd rusty_counter
```

Build the project:

```bash
cargo build
```

Or build and run in one go:

```bash
cargo run
```

## 🧩 How to Play

1. Run the program
```bash
cargo run
```

2. Enter the guess when prompted. You can guess between 1 and 100.
   * `1` Add 1 to counter
   * `2` Remove 1 from counter
   * `3` Quit the counter

Example:

```bash
=================
⏱️ RUSTY COUNTER!
=================

Hint: Enter pure numbers e.g. 1

Here are available options:
1. Add 1 to counter
2. Remove 1 from counter
3. Quit the counter

===== COUNTER: 0 =====

Please enter your option: 1
> Added 1 to counter
```