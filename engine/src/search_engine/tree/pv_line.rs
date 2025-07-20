use crate::search_engine::tree::node::Node;

#[derive(Debug, Default)]
pub struct PvLine(Vec<Node>);
impl PvLine {
    #[inline]
    pub fn new(node: &Node) -> Self {
        Self(vec![node.clone()])
    }

    #[inline]
    pub fn add_node(&mut self, node: &Node) {
        self.0.insert(0, node.clone());
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn score(&self) -> f32 {
        1.0 - self.0[0].score()
    }

    #[inline]
    pub fn to_string(&self, chess960: bool) -> String {
        let mut result = String::new();
        for node in &self.0 {
            result.push_str(node.mv().to_string(chess960).as_str());
            result.push(' ');
        }

        result.trim().to_string()
    }
}