// Definition of TreeNode struct
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to calculate the maximum depth of the binary tree
fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            // Recursively calculate the depth of the left and right subtrees
            let left_depth = max_depth(node.left.as_ref());
            let right_depth = max_depth(node.right.as_ref());
            
            // Return the maximum depth of the left and right subtrees, plus 1 for the current node
            1 + left_depth.max(right_depth)
        }
        None => 0, // Base case: if the root is None, the depth is 0
    }
}

fn main() {
    // Example binary tree:
    //        1
    //       / \
    //      2   3
    //     / \
    //    4   5
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })),
    }));

    // Calculate and print the maximum depth of the binary tree
    let depth = max_depth(root.as_ref());
    println!("The maximum depth of the binary tree is: {}", depth);
}
