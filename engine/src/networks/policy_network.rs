use chess::{ChessBoard, Move};

use crate::networks::{inputs::Standard768, layers::NetworkLayer};

const INPUT_SIZE: usize = Standard768::input_size();

#[repr(C)]
#[derive(Debug)]
pub struct PolicyNetwork {
    subnets: [PolicyNetworkSubnet; 192],
}

#[repr(C)]
#[derive(Debug)]
pub struct PolicyNetworkSubnet {
    l0: NetworkLayer<f32, INPUT_SIZE, 32>,
    l1: NetworkLayer<f32, 32, 32>,
}

impl PolicyNetwork {
    pub fn get_inputs(&self, board: &ChessBoard) -> Vec<usize> {
        let mut result = Vec::with_capacity(board.occupancy().pop_count() as usize);
        Standard768::map_inputs(board, |idx| result.push(idx));
        result
    }

    pub fn forward(&self, inputs: &Vec<usize>, mv: Move, vertical_flip: u8) -> f32 {
        let see_idx = usize::from(false);

        let from_idx = usize::from(mv.get_from_square() ^ vertical_flip);
        let to_idx = usize::from(mv.get_to_square() ^ vertical_flip) + 64 + see_idx * 64;

        let from = self.subnets[from_idx].forward(inputs);
        let to = self.subnets[to_idx].forward(inputs);

        dot(&from, &to)
    }
}

impl PolicyNetworkSubnet {
    pub fn forward(&self, inputs: &Vec<usize>) -> Vec<f32> {
        let mut l0_out = *self.l0.biases();

        for &input_index in inputs {
            for (bias, weight) in l0_out.values_mut().iter_mut().zip(self.l0.weights()[input_index].values()) {
                *bias += *weight;
            }
        }

        let mut out = *self.l1.biases();
        for (neuron, weights) in l0_out.values().iter().zip(self.l1.weights().iter()) {
            out.madd(relu(*neuron), weights);
        }

        out.values().to_vec()
    }
}

fn dot(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    let mut result = 0.0;

    for (value_a, value_b) in a.iter().zip(b) {
        result += relu(*value_a) * relu(*value_b)
    }

    result
}

fn relu(x: f32) -> f32 {
    x.max(0.0)
}