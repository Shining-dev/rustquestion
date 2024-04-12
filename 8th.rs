//Given a binary tree, implement a function that returns the maximum depth of the tree
#[derive(Debug)]
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left.as_ref());
            let right_depth = max_depth(node.right.as_ref());
            1 + left_depth.max(right_depth)
        }
    }
}

fn main() {
    // Constructing a sample binary tree
    let root = Some(Box::new(TreeNode {
        left: Some(Box::new(TreeNode {
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            left: Some(Box::new(TreeNode {
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                left: None,
                right: None,
            })),
        })),
    }));

    let depth = max_depth(root.as_ref());
    println!("Maximum depth of the binary tree: {}", depth);
}
