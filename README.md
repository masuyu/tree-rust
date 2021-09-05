# tree-rust

Rust プログラミングの練習のため木構造を生成・操作する単純なライブラリを作成する。

## 二分木

単純な二分木を生成します。

```rust
use tree::binary_tree::BinaryTree;

fn main() {
    let tree = BinaryTree::create(vec![3, 13, 23, 2]);
    dbg!(&tree);
    // [src/main.rs:5] &tree = BinaryTree {
    //     root: Some(
    //         Node {
    //             val: 13,
    //             left: Some(
    //                 Node {
    //                     val: 3,
    //                     left: Some(
    //                         Node {
    //                             val: 2,
    //                             left: None,
    //                             right: None,
    //                         },
    //                     ),
    //                     right: None,
    //                 },
    //             ),
    //             right: Some(
    //                 Node {
    //                     val: 23,
    //                     left: None,
    //                     right: None,
    //                 },
    //             ),
    //         },
    //     ),
    // }

    let result = tree.search(3);
    dbg!(&result);
    // [src/main.rs:8] &result = Some(
    //     3,
    // )

    let result = tree.search(999);
    dbg!(&result);
    // [src/main.rs:11] &result = None
}
```
