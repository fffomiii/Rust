/// Узел двоичного дерева.
#[derive(Debug)]
struct Node<T: Ord > {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// Возможно пустое поддерево.
#[derive(Debug)]
struct Subtree<T: Ord >(Option<Box<Node<T>>>);

/// Контейнер сохраняющий множество знаяений, с помощью двоичного дерева.
///
/// Если одно значение добавляется несколько раз, сохраняется только одно.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord > BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Реализуйте методы new, insert, len, и has для Subtree.
impl<T: Ord > Subtree<T> {
    fn new() -> Self {
        Subtree(None)
        //BinaryTree{root: Subtree(Some(Box::new(Node{..Default::default()})))}
    }
    
    fn insert(&mut self, value: T) {
        match &mut self.0{
            None => self.0 = Some(Box::new(Node{value: value, left: Subtree(None), right: Subtree(None)})),
            Some(node) => if node.value > value {node.left.insert(value) } else if node.value < value {node.right.insert(value) } else {},
        }
    }

    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => if node.value == *value {true} else  {node.left.has(value) || node.right.has(value)},
        }
    }

    fn len(&self) -> usize {
        match &self.0{
            None => 0,
            Some(node) => 1 + node.left.len() + node.right.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
