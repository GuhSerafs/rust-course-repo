pub enum BinaryTree<T>{
    Empty, 
    NonEmpty(Box<TreeNode<T>>),
}

pub struct TreeNode<T> {
    pub element: T,
    pub left: BinaryTree<T>,
    pub right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value, 
                    left: BinaryTree::Empty, 
                    right: BinaryTree::Empty
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryTree, BinaryTree::*, TreeNode};

    #[test]
    fn it_works() {
        let jupiter_tree = NonEmpty(Box::new(TreeNode {
            element: "Jupiter",
            left: Empty,
            right: Empty,
        }));
    
        let mercury_tree = NonEmpty(Box::new(TreeNode {
            element: "Mercury",
            left: Empty,
            right: Empty,
        }));
    
        let venus_tree = NonEmpty(Box::new(TreeNode {
            element: "Venus",
            left: Empty,
            right: Empty,
        }));
    
        let mars_tree = NonEmpty(Box::new(TreeNode {
            element: "Mars",
            left: jupiter_tree,
            right: mercury_tree,
        }));
    
        let uranus_tree = NonEmpty(Box::new(TreeNode {
            element: "Uranus",
            left: Empty,
            right: venus_tree,
        }));
    
        let tree = NonEmpty(Box::new(TreeNode {
            element: "Saturn",
            left: mars_tree,
            right: uranus_tree,
        }));

        assert!(matches!(tree, BinaryTree::NonEmpty(_)));
    }

    #[test]
    fn it_really_works() {
        let planets = ["Saturn", "Uranus", "Mars", "Venus", "Mercury", "Jupiter"];
        let mut tree = BinaryTree::Empty;
        for planet in planets {
            tree.add(planet);
        }

        assert!(matches!(tree, BinaryTree::NonEmpty(_)));
    }
}