use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryTree<T> {
    Nil,
    Node {
        val: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

pub fn make(v: Vec<i32>) -> BinaryTree<i32> {
    let mut uniq_v: Vec<_> = v.into_iter().unique().collect();
    let v_len = uniq_v.len();
    if v_len == 0 {
        return BinaryTree::<i32>::Nil;
    }

    uniq_v.sort();
    let center = v_len / 2;
    BinaryTree::<i32>::Node {
        val: uniq_v[center],
        left: recirsive_make(uniq_v[0..center].to_vec()),
        right: recirsive_make(uniq_v[center + 1..].to_vec()),
    }
}

fn recirsive_make(v: Vec<i32>) -> Box<BinaryTree<i32>> {
    let v_len = v.len();
    let center = v_len / 2;

    match v_len {
        0 => Box::new(BinaryTree::<i32>::Nil),
        1 => Box::new(BinaryTree::<i32>::Node {
            val: v[0],
            left: Box::new(BinaryTree::<i32>::Nil),
            right: Box::new(BinaryTree::<i32>::Nil),
        }),
        2 => Box::new(BinaryTree::<i32>::Node {
            val: v[center],
            left: recirsive_make(v[0..center].to_vec()),
            right: Box::new(BinaryTree::<i32>::Nil),
        }),
        _ => Box::new(BinaryTree::<i32>::Node {
            val: v[center],
            left: recirsive_make(v[0..center].to_vec()),
            right: recirsive_make(v[center + 1..].to_vec()),
        }),
    }
}
