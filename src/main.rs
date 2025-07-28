


#[derive(Debug, PartialEq)]
struct Node<T> {
    root: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn new(root: T) -> Self {
        Self {
            root,
            left: None,
            right: None,
        }
    }
    fn build_tree(seq: &[i32], idx: &mut i32) -> Option<Box<Node<i32>>> {
        *idx += 1;

        if *idx >= seq.len() as i32 || seq[*idx as usize] == -1  {
            return None;
        }
        // let root = seq[*idx as usize];
        // let left = Self::build_tree(seq, idx);
        // let right = Self::build_tree(seq, idx);
        let mut root = Node::new(seq[*idx as usize]);
        root.left = Node::<i32>::build_tree(seq, idx);
        root.right = Node::<i32>::build_tree(seq, idx);

        Some(Box::new(root))
    }
    fn preorder(&self)
    where
        T: std::fmt::Debug,
    {
        print!("{:?}", self.root);
        
        if let Some(ref left ) = self.left {
            left.preorder();
        }

        if let Some(ref right) = self.right {
            right.preorder();
        }
    }
}

fn main() {
    let seq = [1, 2, -1, -1, 3, 4, -1, -1, 5, -1, -1];

    let mut idx = -1;

    let tree = Node::<i32>::build_tree(&seq, &mut idx).unwrap();

    print!("{:?}",tree.preorder());
}
