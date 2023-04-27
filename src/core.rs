use crate::util as ul;
use rand::prelude::*;

/// 学習率 TODO 調整する必要がありますか?
pub const NET_LEARNING_RATE: f32 = 0.2f32;
/// Sigmoid関数スケーリング TODO 何で必要
pub const ACTIVATION_RESPONSE: f32 = 0.7f32;

/// 神経細胞
struct NeuralCell {
    /// アクティベーション
    activation: f32,
    /// 誤差
    deviation: f32,
    /// 重み
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
                    let rand_f32: f32 = rng.gen();
                    new_weights.push(rand_f32 - 0.5f32);
                }
                new_weights
            },
        }
    }
}

/// 神経層
pub struct NeuralLayer {
    /// インプット個数
    input_numbers: usize,
    /// 神経細胞個数
    cell_numbers: usize,
    /// 経細胞組
    cells_vec: Vec<NeuralCell>,
}

impl NeuralLayer {
    pub fn new(param_input_number: usize, param_cell_numbers: usize, rng: &mut ThreadRng) -> NeuralLayer {
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

    /// 神経レイヤーをリセットする
    fn reset_layer(&mut self, rng: &mut ThreadRng) {
        self.cells_vec = Vec::with_capacity(self.cell_numbers);
        for index in 0..self.cell_numbers {
            self.cells_vec[index] = NeuralCell::new(self.input_numbers, rng);
        }
    }

    /// 現在のニューラルレイヤーのすべての誤差をクリアします
    fn clear_all_deviations(&mut self) {
        for cell in &mut self.cells_vec {
            cell.deviation = 0.0f32;
        }
    }

    /// 現在のレイヤーのすべての出力値を取得する
    fn get_layer_activation_vec(&self) -> Vec<f32> {
        let mut layer_activation_vec = Vec::with_capacity(self.cell_numbers);
        for index in 0..self.cell_numbers {
            layer_activation_vec.push(self.cells_vec[index].activation);
        }
        layer_activation_vec
    }
}

/// ニューラル ネットワーク
pub struct NeuralNet {
    /// 出力レイヤー数
    output_layer_numbers: usize,
    /// 隠れレイヤーの数（インプットレイヤーは含まれていない）
    hidden_layers_size: usize,
    /// ネットの誤差合計数（すべて誤差分散の合計)
    total_deviation: f32,
    /// 神経レイヤー（インプットレイヤーは含まれていない，最後は出力レイヤー）
    layers_vec: Vec<NeuralLayer>,
}

impl NeuralNet {
    pub fn new(param_output_layer_numbers: usize, param_hidden_layers_size: usize) -> NeuralNet {
        NeuralNet {
            output_layer_numbers: param_output_layer_numbers,
            hidden_layers_size: param_hidden_layers_size,
            total_deviation: 9999.0f32,
            layers_vec: Vec::new(),
        }
    }

    /// 訓練
    pub fn training(&mut self, inputs: &Vec<f32>, targets: &Vec<f32>) {
        // 順伝播
        self.training_update(inputs, targets);
        // 逆伝播
        for index in (0usize..=self.hidden_layers_size).rev() {
            match index {
                0 => self.training_layer(index, inputs),
                _ => {
                    self.layers_vec[index - 1].clear_all_deviations();
                    let pre_activations: Vec<f32> = self.layers_vec[index - 1].get_layer_activation_vec();
                    self.training_layer(index, &pre_activations);
                }
            };
        }
    }

    /// 数字認識
    pub fn determination(&mut self, inputs: &Vec<f32>) -> Vec<f32> {
        let mut next_inputs: Vec<f32> = inputs.clone();
        for index in 0..=self.hidden_layers_size {
            self.update_layer(index, &next_inputs);
            next_inputs = self.layers_vec[index].get_layer_activation_vec();
        }
        next_inputs
    }

    /// ニューラル ネットワークをリセットする
    pub fn reset(&mut self, rng: &mut ThreadRng) {
        for index in 0..=self.hidden_layers_size {
            let layer = &mut self.layers_vec[index];
            layer.reset_layer(rng);
        }
        self.total_deviation = 9999.0f32;
    }

    /// ニューラルレイヤーを追加する
    pub fn push_neural_layer(&mut self, layer: NeuralLayer) {
        self.layers_vec.push(layer);
    }

    /// トレーニング モードでネットワークを更新します (出力層の各ニューロンの出力エラーを更新します)。
    fn training_update(&mut self, inputs: &Vec<f32>, targets: &Vec<f32>) {
        // 順伝播
        let mut next_inputs: Vec<f32> = inputs.clone();
        for index in 0..=self.hidden_layers_size {
            self.update_layer(index, &next_inputs);
            if index < self.hidden_layers_size {
                next_inputs = self.layers_vec[index].get_layer_activation_vec();
            }
        }
        // 出力レイヤー誤差のリセット
        let output_layer = &mut self.layers_vec[self.hidden_layers_size];
        let mut total_deviation_temp: f32 = 0.0f32;
        for index in 0..self.output_layer_numbers {
            let mut cell = &mut output_layer.cells_vec[index];
            cell.deviation = targets[index] - cell.activation;
            total_deviation_temp += cell.deviation.powi(2);
        }
        self.total_deviation = total_deviation_temp;
    }

    /// ニューロンレイヤーを訓練する
    fn training_layer(&mut self, layer_index: usize, pre_activations: &Vec<f32>) {
        let layer: &mut NeuralLayer = &mut self.layers_vec[layer_index];
        let mut pre_layer_update_vec: Vec<(usize, f32)> = Vec::new();
        // 現在の層の神経細胞をトラバースし、各神経細胞の出力誤差と重みを調整する根拠を計算する
        for index_cell in 0..layer.cell_numbers {
            let cell: &mut NeuralCell = &mut layer.cells_vec[index_cell];
            // 逆伝播関数を使用して逆伝播誤差を計算する
            let deviation: f32 = cell.deviation * ul::back_propagation(cell.activation);
            // 現在のニューロンのすべての重みをトラバースし、逆伝播に基づいて誤差と学習率などのパラメータを考慮して新しい重みを計算する。
            for index_weight in 0..layer.input_numbers {
                // ニューロンの前のレイヤーで更新する必要がある重みを記録します
                match layer_index {
                    0 => {}
                    _ => {
                        pre_layer_update_vec.push((index_weight, cell.weights[index_weight] * deviation))
                    }
                };
                // 現在のニューロン バイアス アイテムの重みを更新します
                let update_value: f32 = cell.weights[index_weight] + deviation * NET_LEARNING_RATE * pre_activations[index_weight];
                cell.weights[index_weight] = update_value;
                // オーバーフィッティングを防ぐためにノイズを追加する
                // TODO アンダーフィッティングとオーバーフィッティング ？は何故発生するか
                if index_weight == (layer.input_numbers - 1) {
                    let update_value: f32 = cell.weights[index_weight] + deviation * NET_LEARNING_RATE;
                    cell.weights[index_weight] = update_value;
                }
            }
        }
        // 前のレイヤーのニューロン バイアス アイテムの重みを更新します
        if layer_index > 0 && !pre_layer_update_vec.is_empty() {
            let pre_layer: &mut NeuralLayer = &mut self.layers_vec[layer_index - 1];
            for update_src in &pre_layer_update_vec {
                let mut cell = &mut pre_layer.cells_vec[update_src.0];
                cell.deviation = update_src.1;
            }
        }
    }

    /// 入力に基づいてすべてのニューロンの出力を更新します
    fn update_layer(&mut self, layer_index: usize, inputs: &Vec<f32>) {
        let layer: &mut NeuralLayer = &mut self.layers_vec[layer_index];
        for index_cell in 0..layer.cell_numbers {
            let mut cell: &mut NeuralCell = &mut layer.cells_vec[index_cell];
            let weights: &Vec<f32> = &cell.weights;
            let mut cell_input_total: f32 = 0.0f32;
            for index_weight in 0..layer.input_numbers {
                cell_input_total += weights[index_weight] * inputs[index_weight];
            }
            // オーバーフィッティングを防ぐためにノイズを追加する
            cell_input_total += weights[layer.input_numbers - 1];
            // 出力値を計算する
            cell.activation = ul::sigmoid_activation(cell_input_total, ACTIVATION_RESPONSE);
        }
    }
}
