use utils::AlignString;

use crate::search_engine::tree::Tree;

impl Tree {
    pub fn draw_tree<const FLIP_SCORE: bool>(&self, depth: Option<u8>, node_idx: Option<usize>) {
        let depth = depth.unwrap_or(1);
        let node_idx = node_idx.unwrap_or(0);

        self.print_branch::<FLIP_SCORE>(node_idx, 0, depth, String::new(), false, true, false);
    }

    fn print_branch<const FLIP_SCORE: bool>(&self, node_idx: usize, depth: u8, max_depth: u8, mut prefix: String, is_last: bool, is_root: bool, mut flip: bool) {
        self.print_node(node_idx, &prefix, is_root, is_last, flip && FLIP_SCORE);

        if FLIP_SCORE {
            flip = !flip;
        }

        if depth >= max_depth {
            return;
        }

        if !is_root {
            let new_prefix = if is_last { "    " } else { "│   " }; 
            prefix = format!("{prefix}{new_prefix}")
        }

        let mut children = Vec::new();

        self.get_node(node_idx).map_children(|child_idx| {
            if self.get_node(child_idx).visits() == 0 {
                return;
            }

            children.push(child_idx);
        });

        children.sort_by(|&a, &b| {
            self.get_node(b).visits().cmp(&self.get_node(a).visits())
        });

        for (idx, &child_idx) in (&children).into_iter().enumerate() {
            self.print_branch::<FLIP_SCORE>(child_idx, depth + 1, max_depth, prefix.clone(), idx + 1 == children.len(), false, flip);
        }
    }

    fn print_node(&self, node_idx: usize, prefix: &String, is_root: bool, is_last: bool, flip: bool) {
        let node = self.get_node(node_idx);

        let arrow = if is_root { "" } else {
            if is_last { "└─> " } else { "├─> " }
        };

        let prefix = if is_root {
            if node_idx == 0 { 
                String::from("root") 
            } else { 
                node.mv().to_string(false).align_to_left(5) 
            }
        } else {
            format!("{prefix}{arrow}{}> {}", 
                format!("{:#018x}", node_idx).align_to_right(18), 
                node.mv().to_string(false).align_to_left(5)
            )
        };
        
        let score = if flip { 1.0 - node.score() } else { node.score() };
        let score = format!("{:.2}", score).align_to_right(6);

        let visits = format!("{}", node.visits()).align_to_right(9);

        println!("{prefix}  {score} score  {visits} visits");
    }
}