# tree-rust

Rust プログラミングの練習のため木構造を生成・操作する単純なライブラリを作成する。

## 二分木

単純な二分木を生成します。

```rust
use tree::binary_tree::BinaryTree;

fn main() {
    let tree = BinaryTree::create(vec![3, 13, 23, 2]);
    dbg!(&tree);
    // [src/main.rs:5] tree = Node {
    //     val: 13,
    //     left: Node {
    //         val: 3,
    //         left: Node {
    //             val: 2,
    //             left: Nil,
    //             right: Nil,
    //         },
    //         right: Nil,
    //     },
    //     right: Node {
    //         val: 23,
    //         left: Nil,
    //         right: Nil,
    //     },
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
