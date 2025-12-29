# 🦀 Rust DSA - LeetCode Solutions

A comprehensive collection of LeetCode problem solutions implemented in Rust, focusing on data structures and algorithms. This repository serves as a daily practice ground for mastering Rust while solving algorithmic challenges.

## 🚀 Features

- **Daily LeetCode Practice**: Systematic approach to solving LeetCode problems
- **Rust-First Solutions**: All solutions written in idiomatic Rust
- **Organized Structure**: Problems categorized by topic (Arrays, Binary Search, Linked Lists, etc.)
- **Test Coverage**: Each solution includes test cases
- **Helper Scripts**: Automated file creation for new problems
- **Zero Dependencies**: Pure Rust standard library implementations

## 📁 Project Structure

```
rust-dsa/
├── src/
│   ├── arrays/          # Array manipulation problems
│   ├── binary_search/   # Binary search algorithms
│   ├── leetcode/        # LeetCode problem solutions (p00001 - p00023+)
│   ├── linked_list/     # Linked list problems and utilities
│   └── main.rs          # Helper script for creating new problem files
└── Cargo.toml
```

## 🛠️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone <your-repo-url>
cd rust-dsa
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## 📝 Usage

### Running a Specific Problem

To run tests for a specific problem:

```bash
cargo test p00001  # Run tests for problem 1
```

### Creating a New Problem File

The project includes a helper script to automatically create new problem files:

```bash
cargo run p00024
```

This will:
- Create `src/leetcode/p00024.rs`
- Automatically update `src/leetcode/mod.rs` with the new module

### Running All Tests

```bash
cargo test
```

### Running with Output

To see test output:

```bash
cargo test -- --nocapture
```

## 📊 Progress

### LeetCode Problems Solved

Currently solving problems **p00001** through **p00023** and counting! 🎯

| Problem # | Title | Category | Status |
|-----------|-------|----------|--------|
| 1 | Two Sum | Hash Map | ✅ |
| 2 | Add Two Numbers | Linked List | ✅ |
| 3 | Longest Substring Without Repeating Characters | Sliding Window | ✅ |
| ... | ... | ... | ... |
| 19 | Remove Nth Node From End of List | Linked List | ✅ |

### Problem Categories

- **Arrays**: Array manipulation, rotation, searching
- **Binary Search**: Binary search variations
- **Linked Lists**: List operations and algorithms
- **LeetCode**: General LeetCode problems

## 🧪 Testing

Each solution includes comprehensive test cases. Example:

```rust
#[test]
fn test1() {
    let ans = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(ans, vec![0, 1]);
}
```

Run all tests:
```bash
cargo test
```

Run specific test:
```bash
cargo test test1
```

## 🎯 Goals

- [ ] Solve 100+ LeetCode problems
- [ ] Master Rust ownership and borrowing
- [ ] Implement efficient algorithms with optimal time/space complexity
- [ ] Build a comprehensive DSA reference in Rust

## 📚 Learning Resources

- [LeetCode](https://leetcode.com/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## 🤝 Contributing

This is a personal learning project, but suggestions and improvements are welcome!

## 📄 License

This project is open source and available for learning purposes.

## 🏆 Daily Streak

Solving LeetCode problems daily to build consistency and improve problem-solving skills! 💪

---

**Happy Coding! 🦀**

*Built with Rust for performance, safety, and fun.*

