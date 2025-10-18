**Read this in other languages: [[English](README_en.md), [中文](README.md)].**
# LeetCode Solutions in Rust

This is a repository containing solutions to LeetCode algorithm problems implemented in the Rust programming language.

## Project Structure

Each LeetCode problem corresponds to a separate source file (formatted as `lc{problem_number}.rs`) located primarily in the `src/` directory.
- `src/main.rs` - Program entry point
- `src/solution.rs` - Defines the common Solution struct
- `src/listnode.rs`, `src/treenode.rs` - Define commonly used data structures
- `src/lc*.rs` - Specific implementations for each problem

## Code Style

- All problem solutions are implemented as methods of the `Solution` struct. Just copy ```impl Solution {}``` to LeetCode website and submit
- Each solution file usually contains corresponding test cases
- Efficient code is written using Rust's ownership, borrowing, and lifetime features

## How to Run

1. Ensure you have the Rust development environment installed

2. Compile the project:
```bash
cargo build
```

3. Run the project:
```bash
cargo run
```

4. Run the tests:
```bash
cargo test
```

## Example

Take problem 14 (Longest Common Prefix) as an example:
```rust
// Implemented in lc14.rs
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // Implementation code
    }
}

// Test case
#[test]
fn test() {
    let res = Solution::longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    assert_eq!(res, "fl".to_string());
}
```

## Continuous Updates

This repository will be continuously updated with more Rust implementations of LeetCode problems.

Happy Coding! 🦀