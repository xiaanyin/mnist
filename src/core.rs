use crate::util::{self as ul, IMAGE_SIZE};
use rand::prelude::*;

/// 神经细胞
struct NeuralCell {
    /// 激活
    activation: f32,
    /// 误差
    deviation: f32,
    /// 权重
    weights: Vec<f32>,
}

impl NeuralCell {
    fn new(input_numbers: usize, rng: &mut ThreadRng) -> NeuralCell {
        NeuralCell {
            activation: 0.0f32,
            deviation: 0.0f32,
            weights: {
                let mut new_weights = Vec::with_capacity(input_numbers);
                for _ in 0..input_numbers {
                    new_weights.push(rng.gen::<f32>() - 0.5f32);
                }
                new_weights
            },
        }
    }
}

/// 神经层
pub struct NeuralLayer {
    /// 每个神经细胞的输入数目
    input_numbers: usize,
    /// 神经细胞数目
    cell_numbers: usize,
    /// 神经细胞数组
    cells_vec: Vec<NeuralCell>,
}

impl NeuralLayer {
    pub fn new(
        param_input_number: usize,
        param_cell_numbers: usize,
        rng: &mut ThreadRng,
    ) -> NeuralLayer {
        NeuralLayer {
            input_numbers: param_input_number,
            cell_numbers: param_cell_numbers,
            cells_vec: {
                let mut new_cell_vec: Vec<NeuralCell> = Vec::with_capacity(param_cell_numbers);
                for _ in 0..param_cell_numbers {
                    new_cell_vec.push(NeuralCell::new(param_input_number, rng));
                }
                new_cell_vec
            },
        }
    }

    /// 重置神经层
    fn reset_layer(&mut self, rng: &mut ThreadRng) {
        self.cells_vec = Vec::with_capacity(self.cell_numbers);
        for index in 0..self.cell_numbers {
            self.cells_vec[index] = NeuralCell::new(self.input_numbers, rng);
        }
    }

    /// 清空当前神经层所有误差
    fn clear_all_deviations(&mut self) {
        for cell in &mut self.cells_vec {
            cell.deviation = 0.0f32;
        }
    }

    /// 获取当前层所有的输出值
    fn get_layer_activation_vec(&self) -> Vec<f32> {
        let mut layer_activation_vec = Vec::with_capacity(self.cell_numbers);
        for index in 0..self.cell_numbers {
            layer_activation_vec.push(self.cells_vec[index].activation);
        }
        layer_activation_vec
    }
}

/// 神经网络
pub struct NeuralNet {
    /// 输出层个数
    output_layer_numbers: usize,
    /// 隐藏层数目（不包含输入输出层）
    hidden_layers_size: usize,
    /// 网络总误差（所有误差的方差和)
    total_deviation: f32,
    /// 神经层（不包含输入层，最后一层为输出层）
    layers_vec: Vec<NeuralLayer>,
    /// 学习率 TODO 该如何调整以达到最优学习率？
    net_learning_rate: f32,
    /// Sigmoid函数缩放率 TODO 如何理解这个参数的作用？
    activation_response: f32,
}

impl NeuralNet {
    /// output_layer_numbers: 输出层个数
    /// hidden_layers_size: hidden_layers_size
    /// net_learning_rate: 学习率
    /// activation_response: Sigmoid函数缩放率
    /// num_hidden: 隐含层神经元个数
    /// rng
    pub fn new(
        output_layer_numbers: usize,
        hidden_layers_size: usize,
        net_learning_rate: f32,
        activation_response: f32,
        num_hidden: usize,
        rng: &mut ThreadRng,
    ) -> NeuralNet {
        NeuralNet {
            output_layer_numbers: output_layer_numbers,
            hidden_layers_size: hidden_layers_size,
            total_deviation: 9999.0f32,
            layers_vec: {
                let mut layers_vec = Vec::with_capacity(hidden_layers_size + 1);
                // 第一层，输入层个数为图片的像素点个数IMAGE_SIZE，神经细胞个数为隐含层神经元个数
                layers_vec.push(NeuralLayer::new(IMAGE_SIZE, num_hidden, rng));
                // 第二层以及以后的隐含层，输入层个数为隐含层神经元个数，神经细胞个数为隐含层神经元个数
                if hidden_layers_size > 1 {
                    for _ in 1..hidden_layers_size {
                        layers_vec.push(NeuralLayer::new(num_hidden, num_hidden, rng));
                    }
                }
                // 输出层，输入个数为隐含层神经元个数，神经细胞个数为10
                layers_vec.push(NeuralLayer::new(num_hidden, output_layer_numbers, rng));
                layers_vec
            },
            net_learning_rate: net_learning_rate,
            activation_response: activation_response,
        }
    }

    /// 训练
    pub fn training(&mut self, inputs: &Vec<f32>, targets: &Vec<f32>) {
        // 正向传播
        self.training_update(inputs, targets);
        // 反向传播
        for index in (0usize..=self.hidden_layers_size).rev() {
            match index {
                0 => self.training_layer(index, inputs),
                _ => {
                    self.layers_vec[index - 1].clear_all_deviations();
                    let pre_activations: Vec<f32> =
                        self.layers_vec[index - 1].get_layer_activation_vec();
                    self.training_layer(index, &pre_activations);
                }
            };
        }
    }

    /// 数字识别
    pub fn determination(&mut self, inputs: &Vec<f32>) -> Vec<f32> {
        let mut next_inputs: Vec<f32> = inputs.clone();
        for index in 0..=self.hidden_layers_size {
            self.update_layer(index, &next_inputs);
            next_inputs = self.layers_vec[index].get_layer_activation_vec();
        }
        next_inputs
    }

    /// 重置神经网络
    pub fn reset(&mut self, rng: &mut ThreadRng) {
        for index in 0..=self.hidden_layers_size {
            let layer = &mut self.layers_vec[index];
            layer.reset_layer(rng);
        }
        self.total_deviation = 9999.0f32;
    }

    /// 增加神经层
    pub fn push_neural_layer(&mut self, layer: NeuralLayer) {
        self.layers_vec.push(layer);
    }

    /// 以训练模式更新网络（更新输出层的每个神经细胞的输出误差）
    fn training_update(&mut self, inputs: &Vec<f32>, targets: &Vec<f32>) {
        // 正向传播
        let mut next_inputs: Vec<f32> = inputs.clone();
        for index in 0..=self.hidden_layers_size {
            self.update_layer(index, &next_inputs);
            if index < self.hidden_layers_size {
                next_inputs = self.layers_vec[index].get_layer_activation_vec();
            }
        }
        // 重置输出层误差
        let output_layer = &mut self.layers_vec[self.hidden_layers_size];
        let mut total_deviation_temp: f32 = 0.0f32;
        for index in 0..self.output_layer_numbers {
            let mut cell = &mut output_layer.cells_vec[index];
            cell.deviation = targets[index] - cell.activation;
            total_deviation_temp += cell.deviation.powi(2);
        }
        self.total_deviation = total_deviation_temp;
    }

    /// 训练神经细胞层
    fn training_layer(&mut self, layer_index: usize, pre_activations: &Vec<f32>) {
        let layer: &mut NeuralLayer = &mut self.layers_vec[layer_index];
        let mut pre_layer_update_vec: Vec<(usize, f32)> = Vec::new();
        // 遍历当前层的神经细胞，并计算每个神经细胞的输出误差以及调整权重的依据
        for index_cell in 0..layer.cell_numbers {
            let cell: &mut NeuralCell = &mut layer.cells_vec[index_cell];
            // 利用反向传播函数计算反向传播回来的误差
            let deviation: f32 = cell.deviation * ul::back_propagation(cell.activation);
            // 遍历当前神经细胞的所有权重，并基于反向传播回来的误差和学习率等参数计算新的权重值
            for index_weight in 0..layer.input_numbers {
                // 记录前一层神经细胞需要更新的权重
                match layer_index {
                    0 => {}
                    _ => pre_layer_update_vec
                        .push((index_weight, cell.weights[index_weight] * deviation)),
                };
                // 更新当前神经细胞偏置项的权重
                let update_value: f32 = cell.weights[index_weight]
                    + deviation * self.net_learning_rate * pre_activations[index_weight];
                cell.weights[index_weight] = update_value;
                // 增加噪音，防止过拟合
                // TODO 什么样的情况会导致过拟合与拟合不足
                if index_weight == (layer.input_numbers - 1) {
                    let update_value: f32 =
                        cell.weights[index_weight] + deviation * self.net_learning_rate;
                    cell.weights[index_weight] = update_value;
                }
            }
        }
        // 更新前一层神经细胞偏置项的权重
        if layer_index > 0 && !pre_layer_update_vec.is_empty() {
            let pre_layer: &mut NeuralLayer = &mut self.layers_vec[layer_index - 1];
            for update_src in &pre_layer_update_vec {
                let mut cell = &mut pre_layer.cells_vec[update_src.0];
                cell.deviation = update_src.1;
            }
        }
    }

    /// 根据输入更新所有神经细胞的输出
    fn update_layer(&mut self, layer_index: usize, inputs: &Vec<f32>) {
        let layer: &mut NeuralLayer = &mut self.layers_vec[layer_index];
        for index_cell in 0..layer.cell_numbers {
            let mut cell: &mut NeuralCell = &mut layer.cells_vec[index_cell];
            let weights: &Vec<f32> = &cell.weights;
            let mut cell_input_total: f32 = 0.0f32;
            for index_weight in 0..layer.input_numbers {
                cell_input_total += weights[index_weight] * inputs[index_weight];
            }
            // 增加噪音，防止过拟合
            cell_input_total += weights[layer.input_numbers - 1];
            // 计算输出值
            cell.activation = ul::sigmoid_activation(cell_input_total, self.activation_response);
        }
    }
}
