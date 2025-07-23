use utils::{bytes_to_string, heat_color, number_to_string, AlignString, Colors, Theme};

use crate::search_engine::tree::{node::Node, GameState, Tree};

impl Tree {
    pub fn draw_tree<const FLIP_SCORE: bool>(&self, depth: Option<u8>, node_idx: Option<usize>) {
        let depth = depth.unwrap_or(1);
        let node_idx = node_idx.unwrap_or(self.root_index());

        let tree_size = self.tree_size();
        let current_size = self.current_index().min(tree_size);

        let current_size_nodes = number_to_string(current_size as u128).secondary(0.0);
        let tree_size_nodes = number_to_string(tree_size as u128).secondary(0.0);

        let current_size_mem = format!(
            "{}B",
            bytes_to_string((current_size * std::mem::size_of::<Node>()) as u128)
        )
        .secondary(3.0 / 29.0);
        let tree_size_mem = format!(
            "{}B",
            bytes_to_string((tree_size * std::mem::size_of::<Node>()) as u128)
        )
        .secondary(3.0 / 29.0);

        let usage = current_size as f32 / tree_size as f32;
        let usage = heat_color(
            format!("{:.2}%", usage * 100.0).as_str(),
            1.0 - usage,
            0.0,
            1.0,
        );

        println!(
            "{}",
            format!(
                "\nNodes:  {}{}{}",
                current_size_nodes,
                "/".gray(),
                tree_size_nodes
            )
            .primary(0.0)
        );
        println!(
            "{}",
            format!(
                "Memory: {}{}{}",
                current_size_mem,
                "/".gray(),
                tree_size_mem
            )
            .primary(3.0 / 29.0)
        );
        println!("{}", format!("Usage:  {usage}\n").primary(5.0 / 29.0));

        let node_depth = self.find_node_depth(self.root_index(), node_idx);

        if node_depth.is_none() {
            return;
        }

        self.print_branch::<FLIP_SCORE>(
            node_idx,
            0,
            depth,
            String::new(),
            false,
            true,
            node_depth.unwrap() % 2 == 0,
            0,
            20,
        );
    }

    fn print_branch<const FLIP_SCORE: bool>(
        &self,
        node_idx: usize,
        depth: u8,
        max_depth: u8,
        mut prefix: String,
        is_last: bool,
        is_root: bool,
        mut flip: bool,
        iter_idx: usize,
        iter_size: usize,
    ) {
        self.print_node(
            node_idx,
            &prefix,
            is_root,
            is_last,
            flip && FLIP_SCORE,
            iter_idx,
            iter_size,
        );

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

        children.sort_by(|&a, &b| self.get_node(b).visits().cmp(&self.get_node(a).visits()));

        for (idx, &child_idx) in (&children).into_iter().enumerate() {
            self.print_branch::<FLIP_SCORE>(
                child_idx,
                depth + 1,
                max_depth,
                prefix.clone(),
                idx + 1 == children.len(),
                false,
                flip,
                idx,
                children.len(),
            );
        }
    }

    fn print_node(
        &self,
        node_idx: usize,
        prefix: &String,
        is_root: bool,
        is_last: bool,
        flip: bool,
        iter_idx: usize,
        iter_size: usize,
    ) {
        let node = self.get_node(node_idx);
        let color_gradient = (iter_idx + 5) as f32 / (iter_size + 10) as f32;

        let arrow = if is_root {
            ""
        } else {
            if is_last {
                "└─> "
            } else {
                "├─> "
            }
        };

        let prefix = if is_root {
            if node_idx == 0 {
                String::from("root")
            } else {
                node.mv().to_string(false).align_to_left(5)
            }
            .primary(color_gradient)
        } else {
            format!(
                "{prefix}{arrow}{}{} {}",
                format!("{:#018x}", node_idx)
                    .align_to_right(18)
                    .primary(color_gradient),
                ">".secondary(color_gradient),
                node.mv()
                    .to_string(false)
                    .align_to_left(5)
                    .primary(color_gradient)
            )
        }
        .white();

        let score = if flip {
            1.0 - node.score()
        } else {
            node.score()
        };
        let score = heat_color(&format!("{:.2}", score).align_to_right(6), score, 0.0, 1.0);

        let visits = format!("{}", node.visits()).align_to_right(9);

        let state = match node.state() {
            GameState::Draw => String::from("DRAW"),
            GameState::Win(len) => format!("WIN IN {len}"),
            GameState::Loss(len) => format!("LOSS IN {len}"),
            _ => String::new(),
        };

        println!(
            "{}",
            format!(
                "{prefix}  {score} score  {} visits  {}",
                visits.to_string().white(),
                state.white()
            )
            .secondary(color_gradient)
        );
    }
}
