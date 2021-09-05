use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl BinaryTree<i32> {
    pub fn create(v: Vec<i32>) -> BinaryTree<i32> {
        let mut uniq_v: Vec<_> = v.into_iter().unique().collect();
        let v_len = uniq_v.len();
        if v_len == 0 {
            return BinaryTree { root: None };
        }

        uniq_v.sort();
        let center = v_len / 2;
        BinaryTree {
            root: Some(Box::new(Node {
                val: uniq_v[center],
                left: Self::create_recirsive_helper(uniq_v[0..center].to_vec()),
                right: Self::create_recirsive_helper(uniq_v[center + 1..].to_vec()),
            })),
        }
    }

    fn create_recirsive_helper(v: Vec<i32>) -> Option<Box<Node<i32>>> {
        let v_len = v.len();
        let center = v_len / 2;

        match v_len {
            0 => None,
            1 => Some(Box::new(Node {
                val: v[0],
                left: None,
                right: None,
            })),
            2 => Some(Box::new(Node {
                val: v[center],
                left: Self::create_recirsive_helper(v[0..center].to_vec()),
                right: None,
            })),
            _ => Some(Box::new(Node {
                val: v[center],
                left: Self::create_recirsive_helper(v[0..center].to_vec()),
                right: Self::create_recirsive_helper(v[center + 1..].to_vec()),
            })),
        }
    }

    pub fn search(&self, number: i32) -> Option<i32> {
        if let Some(ref boxed_node) = self.root {
            boxed_node.search(number)
        } else {
            None
        }
    }
}

impl Node<i32> {
    pub fn search(&self, number: i32) -> Option<i32> {
        if number == self.val {
            Some(number)
        } else if number > self.val {
            if let Some(boxed_node) = &self.right {
                boxed_node.search(number)
            } else {
                None
            }
        } else {
            if let Some(boxed_node) = &self.left {
                boxed_node.search(number)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_tree::BinaryTree;
    use crate::binary_tree::Node;

    #[test]
    fn test_make_from_binary_tree_when_0_element() {
        let expected = BinaryTree { root: None };
        assert_eq!(expected, BinaryTree::create(vec![]));
    }

    #[test]
    fn test_make_from_binary_tree_when_1_element() {
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                val: 3,
                left: None,
                right: None,
            })),
        };
        assert_eq!(expected, BinaryTree::create(vec![3]));
    }

    #[test]
    fn test_make_from_binary_tree_when_2_elements() {
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                val: 123,
                left: Some(Box::new(Node {
                    val: 3,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };
        assert_eq!(expected, BinaryTree::create(vec![3, 123]));
    }

    #[test]
    fn test_make_from_binary_tree_when_3_elements() {
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                val: 3,
                left: Some(Box::new(Node {
                    val: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    val: 4,
                    left: None,
                    right: None,
                })),
            })),
        };
        assert_eq!(expected, BinaryTree::create(vec![3, 1, 4]));
    }

    #[test]
    fn test_make_from_binary_tree_when_more_then_3_elements() {
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                val: 3,
                left: Some(Box::new(Node {
                    val: 2,
                    left: Some(Box::new(Node {
                        val: 1,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: Some(Box::new(Node {
                    val: 4,
                    left: None,
                    right: None,
                })),
            })),
        };
        assert_eq!(expected, BinaryTree::create(vec![3, 1, 4, 2]));
    }

    #[test]
    fn test_search() {
        let expected = Some(3);
        let tree = BinaryTree::create(vec![3, 1, 4, 2]);
        assert_eq!(expected, tree.search(3));
    }

    #[test]
    fn test_search_when_not_found() {
        let expected = None;
        let tree = BinaryTree::create(vec![3, 1, 4, 2]);
        assert_eq!(expected, tree.search(7));
    }

    #[test]
    fn test_search_when_empty_tree() {
        let expected = None;
        let tree = BinaryTree::create(vec![]);
        assert_eq!(expected, tree.search(3));
    }
}
