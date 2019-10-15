/// 神经细胞
struct Neuron {
    /// 激活
    activation: f32,
    /// 误差
    diff: f32,
    /// 权重
    weights: Vec<f32>,
}

impl Neuron {
    fn new() -> Neuron {
        Neuron {
            activation: 0.0f32,
            diff: 0.0f32,
            weights: Vec::new()
        }
    }
}

/// 神经细胞层
struct NeuronLayer {
    /// 每个神经细胞的输入数目
    input_size: usize,
    /// 神经细胞数目
    neuron_size: usize,
    /// 神经细胞数组
    neurons: Vec<Neuron>
}

impl NeuronLayer {
    fn new() -> Neuron {
        // TODO
    }

    fn reset(&mut self) {

    }
}

/// 神经细胞网络
struct  NeuronNet {
    /// 输入层个数
    inputs_size: usize,
    /// 输出层个数
    output_size: usize,
    /// 隐藏层数目（不包含输入输出层）
    hidden_layers_size: usize,
    /// 学习率（太大会出现错误收敛或者无法收敛，太小也可能导致错误收敛，且学习速度变慢）
    /// TODO 理解有待加深
    learning_rate: f32,
}

impl NeuronNet {
    fn new() -> NeuronNet {
        // TODO
    }
}
//struct neuronLayer
//{
//    public:
//    neuronLayer(int numNeurons, int numInputsPerNeuron); /** 神经细胞层的构造函数*/
//neuronLayer(neuronLayer& nl); /** 神经细胞层的拷贝构造函数 */
//~neuronLayer(); /** 神经细胞层的析构函数 */
//void reset(void); /** 神经细胞层的重置函数（将权重等参数都重置为随机值）*/
//public:
//int mNumInputsPerNeuron; /** 当前层的每个神经细胞的输入数目 */
//int mNumNeurons; /** 当前层的神经细胞数目 */
//double** mWeights; /** 2维数组, 行: 代表神经细胞（每一行就是一个神经细胞的所有权重）, 列: 代表神经细胞的输入权重 */
//double* mOutActivations; /** 当前层每个神经细胞的输出值 */
//double* mOutErrors; /** 当前层每个神经细胞的误差值 */
//};
