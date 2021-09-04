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

impl BinaryTree<i32> {
    pub fn create(v: Vec<i32>) -> BinaryTree<i32> {
        let mut uniq_v: Vec<_> = v.into_iter().unique().collect();
        let v_len = uniq_v.len();
        if v_len == 0 {
            return BinaryTree::<i32>::Nil;
        }

        uniq_v.sort();
        let center = v_len / 2;
        BinaryTree::<i32>::Node {
            val: uniq_v[center],
            left: Self::create_recirsive_helper(uniq_v[0..center].to_vec()),
            right: Self::create_recirsive_helper(uniq_v[center + 1..].to_vec()),
        }
    }

    fn create_recirsive_helper(v: Vec<i32>) -> Box<BinaryTree<i32>> {
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
                left: Self::create_recirsive_helper(v[0..center].to_vec()),
                right: Box::new(BinaryTree::<i32>::Nil),
            }),
            _ => Box::new(BinaryTree::<i32>::Node {
                val: v[center],
                left: Self::create_recirsive_helper(v[0..center].to_vec()),
                right: Self::create_recirsive_helper(v[center + 1..].to_vec()),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::BinaryTree;

    #[test]
    fn test_make_from_binary_tree_when_0_element() {
        let expected = BinaryTree::<i32>::Nil;
        assert_eq!(expected, BinaryTree::create(vec![]));
    }

    #[test]
    fn test_make_from_binary_tree_when_1_element() {
        let expected = BinaryTree::<i32>::Node {
            val: 3,
            left: Box::new(BinaryTree::<i32>::Nil),
            right: Box::new(BinaryTree::<i32>::Nil),
        };
        assert_eq!(expected, BinaryTree::create(vec![3]));
    }

    #[test]
    fn test_make_from_binary_tree_when_2_elements() {
        let expected = BinaryTree::<i32>::Node {
            val: 123,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 3,
                left: Box::new(BinaryTree::<i32>::Nil),
                right: Box::new(BinaryTree::<i32>::Nil),
            }),
            right: Box::new(BinaryTree::<i32>::Nil),
        };
        assert_eq!(expected, BinaryTree::create(vec![3, 123]));
    }

    #[test]
    fn test_make_from_binary_tree_when_3_elements() {
        let expected = BinaryTree::<i32>::Node {
            val: 3,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 1,
                left: Box::new(BinaryTree::<i32>::Nil),
                right: Box::new(BinaryTree::<i32>::Nil),
            }),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 4,
                left: Box::new(BinaryTree::<i32>::Nil),
                right: Box::new(BinaryTree::<i32>::Nil),
            }),
        };
        assert_eq!(expected, BinaryTree::create(vec![3, 1, 4]));
    }

    #[test]
    fn test_make_from_binary_tree_when_more_then_3_elements() {
        let expected = BinaryTree::<i32>::Node {
            val: 3,
            left: Box::new(BinaryTree::<i32>::Node {
                val: 2,
                left: Box::new(BinaryTree::<i32>::Node {
                    val: 1,
                    left: Box::new(BinaryTree::<i32>::Nil),
                    right: Box::new(BinaryTree::<i32>::Nil),
                }),
                right: Box::new(BinaryTree::<i32>::Nil),
            }),
            right: Box::new(BinaryTree::<i32>::Node {
                val: 4,
                left: Box::new(BinaryTree::<i32>::Nil),
                right: Box::new(BinaryTree::<i32>::Nil),
            }),
        };
        assert_eq!(expected, BinaryTree::create(vec![3, 1, 4, 2]));
    }
}
