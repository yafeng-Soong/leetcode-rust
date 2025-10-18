# LeetCode Rust 题解

这是一个用 Rust 语言实现的 LeetCode 算法题解仓库。

## 项目结构

每个 LeetCode 题目对应一个单独的源文件（格式为 `lc{题目编号}.rs`），主要位于 `src/` 目录下。
- `src/main.rs` - 程序入口点
- `src/solution.rs` - 定义公共的 Solution 结构体
- `src/listnode.rs`, `src/treenode.rs` - 定义常用的数据结构
- `src/lc*.rs` - 各题目的具体实现

## 代码风格

- 所有题目解法都作为 `Solution` 结构体的方法实现，复制```impl Solution {}```到leetcode即可提交
- 每个解法文件中通常包含对应的测试用例
- 使用 Rust 的所有权、借用和生命周期特性编写高效代码

## 如何运行

1. 确保已安装 Rust 开发环境

2. 编译项目：
```bash
cargo build
```

3. 运行项目：
```bash
cargo run
```

4. 运行测试：
```bash
cargo test
```

## 示例

以第 14 题（最长公共前缀）为例：
```rust
// 在 lc14.rs 中实现
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // 实现代码
    }
}

// 测试用例
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

## 持续更新

本仓库会持续更新，添加更多 LeetCode 题目的 Rust 实现。

Happy Coding! 🦀