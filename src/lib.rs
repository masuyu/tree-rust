pub mod binary_tree;

#[cfg(test)]
mod tests {
    use crate::binary_tree::{make, BinaryTree};

    #[test]
    fn test_make_from_binary_tree_when_0_element() {
        let expected = BinaryTree::<i32>::Nil;
        assert_eq!(expected, make(vec![]));
    }

    #[test]
    fn test_make_from_binary_tree_when_1_element() {
        let expected = BinaryTree::<i32>::Node {
            val: 3,
            left: Box::new(BinaryTree::<i32>::Nil),
            right: Box::new(BinaryTree::<i32>::Nil),
        };
        assert_eq!(expected, make(vec![3]));
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
        assert_eq!(expected, make(vec![3, 123]));
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
        assert_eq!(expected, make(vec![3, 1, 4]));
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
        assert_eq!(expected, make(vec![3, 1, 4, 2]));
    }
}
