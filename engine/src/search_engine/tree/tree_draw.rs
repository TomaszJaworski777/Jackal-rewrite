use utils::{bytes_to_string, heat_color, number_to_string, AlignString, Colors, Theme};

use crate::search_engine::{tree::{GameState, Tree}, Edge};

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
            bytes_to_string(Tree::size_to_bytes(current_size) as u128)
        )
        .secondary(3.0 / 29.0);
        let tree_size_mem = format!(
            "{}B",
            bytes_to_string(Tree::size_to_bytes(tree_size) as u128)
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
            None,
            0,
            depth,
            String::new(),
            false,
            true,
            node_depth.unwrap() % 2 == 0,
            0,
            20,
            0.0,
            0.10
        );
    }

    fn print_branch<const FLIP_SCORE: bool>(
        &self,
        node_idx: usize,
        parent_edge: Option<&Edge>,
        depth: u8,
        max_depth: u8,
        mut prefix: String,
        is_last: bool,
        is_root: bool,
        mut flip: bool,
        iter_idx: usize,
        iter_size: usize,
        mut min_policy: f32,
        mut max_policy: f32
    ) {
        self.print_node(
            node_idx,
            parent_edge,
            &prefix,
            is_root,
            is_last,
            flip && FLIP_SCORE,
            iter_idx,
            iter_size,
            min_policy,
            max_policy
        );

        if FLIP_SCORE {
            flip = !flip;
        }

        if depth >= max_depth {
            return;
        }

        if node_idx == usize::MAX {
            return;
        }

        if !is_root {
            let new_prefix = if is_last { "    " } else { "│   " };
            prefix = format!("{prefix}{new_prefix}")
        }

        let mut children = Vec::new();

        for child_idx in 0..self.nodes[node_idx].children().len() {
            let edge = self.get_child_clone(node_idx, child_idx);
        
            if edge.visits() == 0 {
                continue;
            }

            children.push(edge);   
        }

        children.sort_by(|a, b| b.visits().cmp(&a.visits()));

        min_policy = f32::INFINITY;
        max_policy = f32::NEG_INFINITY;

        for child in &children {
            let policy = child.policy() as f32;
            min_policy = min_policy.min(policy);
            max_policy = max_policy.max(policy);
        }

        for (idx, child) in (&children).into_iter().enumerate() {
            self.print_branch::<FLIP_SCORE>(
                child.node_index(),
                Some(child),
                depth + 1,
                max_depth,
                prefix.clone(),
                idx + 1 == children.len(),
                false,
                flip,
                idx,
                children.len(),
                min_policy,
                max_policy
            );
        }
    }

    fn print_node(
        &self,
        node_idx: usize,
        edge: Option<&Edge>,
        prefix: &String,
        is_root: bool,
        is_last: bool,
        flip: bool,
        iter_idx: usize,
        iter_size: usize,
        min_policy: f32,
        max_policy: f32
    ) {
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

        if edge.is_none() {
            println!(
                "{}",
                format!(
                    "{}  {} visits",
                    "root".primary(color_gradient),
                    format!("{}", self.get_node(node_idx).visits()).align_to_right(9).white(),
                )
                .secondary(color_gradient)
            );

            return;
        }

        let edge = edge.unwrap();

        let node_index = if node_idx == usize::MAX {
            "NULL".to_string()
        } else {
            format!("{:#018x}", node_idx)
        }.align_to_right(18).primary(color_gradient);

        let prefix = format!(
                "{prefix}{arrow}{}{} {}",
                node_index,
                ">".secondary(color_gradient),
                edge.mv()
                    .to_string(false)
                    .align_to_left(5)
                    .primary(color_gradient)
            ).white();

        let score = if flip {
            edge.score().reversed()
        } else {
            edge.score()
        };

        let cp = score.cp(0.5);
        let score = heat_color(&format!("{:.2}", cp as f32 / 100.0).align_to_right(6), score.single(0.5) as f32, 0.0, 1.0);

        let visits = format!("{}", edge.visits()).align_to_right(9);

        let policy = heat_color(&format!("{:.2}%", edge.policy() * 100.0).align_to_right(6), edge.policy() as f32, min_policy, max_policy);

        let state = if node_idx == usize::MAX {
            String::new()
        } else {
            match self.get_node(node_idx).state() {
                GameState::Draw => String::from("DRAW"),
                GameState::Win(len) => format!("WIN IN {len}"),
                GameState::Loss(len) => format!("LOSS IN {len}"),
                _ => String::new(),
            }
        };

        println!(
            "{}",
            format!(
                "{prefix}  {score} score  {} visits  {} policy  {}",
                visits.to_string().white(),
                policy,
                state.white()
            )
            .secondary(color_gradient)
        );
    }
}
