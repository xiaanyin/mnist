#![allow(unused_imports, dead_code, unused_variables)]

extern crate rand;
#[macro_use]
extern crate lazy_static;

mod util;
mod repository;
mod core;

use rand::prelude::*;
use std::collections::HashMap;
use util as ul;
use repository::*;
use crate::util::IMAGE_SIZE;
use crate::core::{NET_LEARNING_RATE, ACTIVATION_RESPONSE};

lazy_static! {
    static ref TARGETS: HashMap<u8, Vec<f32>> = {
        let mut map = HashMap::new();
        map.insert(0, vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        map.insert(1, vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        map.insert(2, vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        map.insert(3, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        map.insert(4, vec![0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        map.insert(5, vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
        map.insert(6, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
        map.insert(7, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        map.insert(8, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        map.insert(9, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        map
    };
}

const TRAIN_IMAGES: &str = "resources/train-images.idx3-ubyte";
const TRAIN_LABELS: &str = "resources/train-labels.idx1-ubyte";
const TEST_IMAGES: &str = "resources/t10k-images.idx3-ubyte";
const TEST_LABELS: &str = "resources/t10k-labels.idx1-ubyte";


// 输出层神经元个数,固定784个(28*28)
//const NUM_NET_IN: usize = 784usize;
// 输出层神经元个数, 0~9, 固定10个
const NUM_NET_OUT: usize = 10usize;
// 隐含层神经元个数， TODO 理解有待加深，如何选择合适的神经元个数？
const NUM_HIDDEN: usize = 400usize;
// 学习次数
const LEARNING_TIMES: usize = 5usize;

fn main() {
    for _ in 0..3 {
        execution();
    }
    println!();
}

fn execution() {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut net = core::NeuralNet::new(NUM_NET_OUT, 1);
    let layer_hidden = core::NeuralLayer::new(IMAGE_SIZE, NUM_HIDDEN, &mut rng);
    let layer_output = core::NeuralLayer::new(NUM_HIDDEN, NUM_NET_OUT, &mut rng);
    net.push_neural_layer(layer_hidden);
    net.push_neural_layer(layer_output);

    for _ in 0..LEARNING_TIMES {
        let mut train_data_provider = DataProvider::new(TRAIN_IMAGES, TRAIN_LABELS);
        loop {
            match train_data_provider.next() {
                None => break,
                Some(data) => {
                    let inputs: Vec<f32> = ul::prepare_inputs_with_noise(&data.0, &mut rng);
                    let targets: &Vec<f32> = TARGETS.get(&data.1).unwrap();
                    net.training(&inputs, targets);
                }
            };
        }
    }

    let mut ok_counts: usize = 0usize;
    let mut test_data_provider = DataProvider::new(TEST_IMAGES, TEST_LABELS);
    loop {
        match test_data_provider.next() {
            None => break,
            Some(data) => {
                let inputs: Vec<f32> = ul::prepare_inputs_no_noise(&data.0);
                let output_layer_activations: Vec<f32> = net.determination(&inputs);
                let max_index: u8 = ul::find_max_index_in_vec(&output_layer_activations);
                if data.1 == max_index {
                    ok_counts += 1;
                }
            }
        };
    }
    println!("隐含层神经元个数=[{}]，学习率=[{}]，激活函数缩放率=[{}]，学习次数=[{}]，测试结果[{}/{}]",
             NUM_HIDDEN, NET_LEARNING_RATE, ACTIVATION_RESPONSE, LEARNING_TIMES, ok_counts, test_data_provider.total);
}


