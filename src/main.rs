#![allow(unused_imports, dead_code, unused_variables)]

extern crate rand;
#[macro_use]
extern crate lazy_static;

mod core;
mod repository;
mod util;

#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::util::IMAGE_SIZE;
use rand::prelude::*;
use repository::*;
use std::collections::HashMap;
use util as ul;

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

// 输入层神经元个数,固定784个(28*28)
//const NUM_NET_IN: usize = 784usize;
// 输出层神经元个数, 0~9, 固定10个
const NUM_NET_OUT: usize = 10usize;

fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("result.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("Starting application");

    let mut params_list: Vec<(usize, usize, usize, f32, f32)> = Vec::new();
    // 隐含层神经元个数
    let vec_num_hidden = vec![30, 60, 80, 100, 120, 150, 200, 250, 300, 350, 400, 450, 500];
    // 隐藏层数目
    let vec_hidden_layers_size = vec![1, 2, 3, 4, 5];
    // 学习次数
    let vec_learning_times = vec![1, 2, 3, 4, 5];
    // 学习率
    let vec_net_learning_rate = vec![0.15f32, 0.2f32, 0.25f32, 0.3f32];
    // Sigmoid函数缩放率
    let vec_activation_response = vec![0.6f32, 0.7f32, 0.8f32];

    for i in &vec_num_hidden {
        for j in &vec_hidden_layers_size {
            for k in &vec_learning_times {
                for l in &vec_net_learning_rate {
                    for m in &vec_activation_response {
                        params_list.push((*i, *j, *k, *l, *m));
                    }
                }
            }
        }
    }

    for params in &params_list {
        for _ in 0..5 {
            execution(params.0, params.1, params.2, params.3, params.4);
        }
    }
}

/// num_hidden: 隐含层神经元个数 TODO 理解有待加深，如何选择合适的神经元个数？
/// hidden_layers_size: 隐藏层数目 （不包含输入输出层）
/// learning_times: 学习次数 TODO 反复学习是否能提高准确率
/// net_learning_rate: 学习率 TODO 该如何调整以达到最优学习率？
/// activation_response: Sigmoid函数缩放率 TODO 如何理解这个参数的作用？
fn execution(
    num_hidden: usize,
    hidden_layers_size: usize,
    learning_times: usize,
    net_learning_rate: f32,
    activation_response: f32,
) {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut net = core::NeuralNet::new(
        NUM_NET_OUT,
        hidden_layers_size,
        net_learning_rate,
        activation_response,
    );
    let layer_hidden = core::NeuralLayer::new(IMAGE_SIZE, num_hidden, &mut rng);
    let layer_output = core::NeuralLayer::new(num_hidden, NUM_NET_OUT, &mut rng);
    net.push_neural_layer(layer_hidden);
    net.push_neural_layer(layer_output);

    for _ in 0..learning_times {
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
    info!(
        "隐含层神经元个数=[{}]，隐藏层数目=[{}]，学习率=[{}]，激活函数缩放率=[{}]，学习次数=[{}]，测试结果[{}/{}]",
        num_hidden,
        hidden_layers_size,
        net_learning_rate,
        activation_response,
        learning_times,
        ok_counts,
        test_data_provider.total
    );
}
