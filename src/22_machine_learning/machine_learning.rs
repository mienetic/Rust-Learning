//! Machine Learning and AI
//!
//! การใช้ Rust สำหรับ Machine Learning และ Artificial Intelligence

use std::collections::HashMap;

/// Vector operations for ML
#[derive(Debug, Clone, PartialEq)]
struct Vector {
    data: Vec<f64>,
}

impl Vector {
    const fn new(data: Vec<f64>) -> Self {
        Self { data }
    }
    
    fn zeros(size: usize) -> Self {
        Self {
            data: vec![0.0; size],
        }
    }
    
    fn ones(size: usize) -> Self {
        Self {
            data: vec![1.0; size],
        }
    }
    
    const fn len(&self) -> usize {
        self.data.len()
    }
    
    fn dot(&self, other: &Self) -> f64 {
        assert_eq!(self.len(), other.len());
        self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum()
    }
    
    fn add(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        let data = self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect();
        Self::new(data)
    }
    
    fn subtract(&self, other: &Self) -> Self {
        assert_eq!(self.len(), other.len());
        let data = self.data.iter().zip(&other.data).map(|(a, b)| a - b).collect();
        Self::new(data)
    }
    
    fn scale(&self, scalar: f64) -> Self {
        let data = self.data.iter().map(|x| x * scalar).collect();
        Self::new(data)
    }
    
    fn magnitude(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
    
    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            self.clone()
        } else {
            self.scale(1.0 / mag)
        }
    }
}

/// Matrix operations for ML
#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        
        // Verify all rows have same length
        for row in &data {
            assert_eq!(row.len(), cols);
        }
        
        Self { data, rows, cols }
    }
    
    fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }
    
    fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Self::new(data)
    }
    
    fn get(&self, row: usize, col: usize) -> f64 {
        if row >= self.rows || col >= self.cols {
            panic!("Matrix index out of bounds: ({}, {}) for {}x{} matrix", row, col, self.rows, self.cols);
        }
        self.data[row][col]
    }
    
    fn set(&mut self, row: usize, col: usize, value: f64) {
        if row >= self.rows || col >= self.cols {
            panic!("Matrix index out of bounds: ({}, {}) for {}x{} matrix", row, col, self.rows, self.cols);
        }
        self.data[row][col] = value;
    }
    
    fn multiply(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows);
        
        let mut result = Self::zeros(self.rows, other.cols);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        
        result
    }
    
    fn multiply_vector(&self, vector: &Vector) -> Vector {
        assert_eq!(self.cols, vector.len());
        
        let mut result = vec![0.0; self.rows];
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i] += self.get(i, j) * vector.data[j];
            }
        }
        
        Vector::new(result)
    }
    
    fn transpose(&self) -> Self {
        let mut data = vec![vec![0.0; self.rows]; self.cols];
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j][i] = self.get(i, j);
            }
        }
        
        Self::new(data)
    }
}

/// Activation functions
struct ActivationFunctions;

impl ActivationFunctions {
    fn sigmoid(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }
    
    fn sigmoid_derivative(x: f64) -> f64 {
        let s = Self::sigmoid(x);
        s * (1.0 - s)
    }
    
    const fn relu(x: f64) -> f64 {
        x.max(0.0)
    }
    
    fn relu_derivative(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else { 0.0 }
    }
    
    fn tanh(x: f64) -> f64 {
        x.tanh()
    }
    
    fn tanh_derivative(x: f64) -> f64 {
        let t = x.tanh();
        t.mul_add(-t, 1.0)
    }
    
    fn softmax(inputs: &[f64]) -> Vec<f64> {
        let max_val = inputs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let exp_values: Vec<f64> = inputs.iter().map(|&x| (x - max_val).exp()).collect();
        let sum: f64 = exp_values.iter().sum();
        exp_values.iter().map(|&x| x / sum).collect()
    }
}

/// Loss functions
struct LossFunctions;

impl LossFunctions {
    fn mean_squared_error(predicted: &[f64], actual: &[f64]) -> f64 {
        assert_eq!(predicted.len(), actual.len());
        
        let sum: f64 = predicted.iter()
            .zip(actual.iter())
            .map(|(p, a)| (p - a).powi(2))
            .sum();
        
        sum / predicted.len() as f64
    }
    
    fn cross_entropy(predicted: &[f64], actual: &[f64]) -> f64 {
        assert_eq!(predicted.len(), actual.len());
        
        let sum: f64 = predicted.iter()
            .zip(actual.iter())
            .map(|(p, a)| -a * p.ln())
            .sum();
        
        sum
    }
    
    fn binary_cross_entropy(predicted: f64, actual: f64) -> f64 {
        -actual.mul_add(predicted.ln(), (1.0 - actual) * (1.0 - predicted).ln())
    }
}

/// Simple Neural Network
struct NeuralNetwork {
    weights: Vec<Matrix>,
    biases: Vec<Vector>,
    learning_rate: f64,
}

impl NeuralNetwork {
    fn new(layer_sizes: &[usize], learning_rate: f64) -> Self {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        
        for i in 0..layer_sizes.len() - 1 {
            let input_size = layer_sizes[i];
            let output_size = layer_sizes[i + 1];
            
            // Initialize weights with small random values
            let mut weight_data = vec![vec![0.0; input_size]; output_size];
            for row in &mut weight_data {
                for weight in row {
                    *weight = (rand::random::<f64>() - 0.5) * 0.5;
                }
            }
            weights.push(Matrix::new(weight_data));
            
            // Initialize biases to zero
            biases.push(Vector::zeros(output_size));
        }
        
        Self {
            weights,
            biases,
            learning_rate,
        }
    }
    
    fn forward(&self, input: &Vector) -> (Vector, Vec<Vector>) {
        let mut activations = vec![input.clone()];
        let mut current = input.clone();
        
        for (weight, bias) in self.weights.iter().zip(&self.biases) {
            // Linear transformation: W * x + b
            let linear = weight.multiply_vector(&current).add(bias);
            
            // Apply activation function (sigmoid)
            let activated = Vector::new(
                linear.data.iter().map(|&x| ActivationFunctions::sigmoid(x)).collect()
            );
            
            activations.push(activated.clone());
            current = activated;
        }
        
        (current, activations)
    }
    
    fn backward(&mut self, input: &Vector, target: &Vector) -> f64 {
        let (output, activations) = self.forward(input);
        
        // Calculate loss
        let loss = LossFunctions::mean_squared_error(&output.data, &target.data);
        
        // Backpropagation
        let mut delta = output.subtract(target);
        
        for i in (0..self.weights.len()).rev() {
            // Use correct activation for gradient calculation
            let prev_activation = if i == 0 {
                input
            } else {
                &activations[i - 1]
            };
            
            // Calculate gradients
            let weight_gradient = self.calculate_weight_gradient(&delta, prev_activation, i);
            let bias_gradient = delta.clone();
            
            // Update weights and biases
            self.update_weights(i, &weight_gradient);
            self.update_biases(i, &bias_gradient);
            
            // Calculate delta for previous layer
            if i > 0 {
                let current_activation = &activations[i - 1];
                delta = self.calculate_previous_delta(&delta, i, current_activation);
            }
        }
        
        loss
    }
    
    fn calculate_weight_gradient(&self, delta: &Vector, activation: &Vector, layer_index: usize) -> Matrix {
        // Gradient should have same dimensions as weight matrix
        let weight_rows = self.weights[layer_index].rows;
        let weight_cols = self.weights[layer_index].cols;
        
        let mut gradient_data = vec![vec![0.0; weight_cols]; weight_rows];
        
        for i in 0..weight_rows.min(delta.len()) {
            for j in 0..weight_cols.min(activation.len()) {
                gradient_data[i][j] = delta.data[i] * activation.data[j];
            }
        }
        
        Matrix::new(gradient_data)
    }
    
    fn calculate_previous_delta(&self, delta: &Vector, layer_index: usize, activation: &Vector) -> Vector {
        let weight_transpose = self.weights[layer_index].transpose();
        let linear_delta = weight_transpose.multiply_vector(delta);
        
        // Apply derivative of activation function
        let derivative_data: Vec<f64> = activation.data.iter()
            .zip(&linear_delta.data)
            .map(|(&a, &d)| d * ActivationFunctions::sigmoid_derivative(a))
            .collect();
        
        Vector::new(derivative_data)
    }
    
    fn update_weights(&mut self, layer_index: usize, gradient: &Matrix) {
        // Ensure gradient matrix has the same dimensions as weight matrix
        if gradient.rows != self.weights[layer_index].rows || gradient.cols != self.weights[layer_index].cols {
            println!("Warning: Gradient matrix size ({}, {}) doesn't match weight matrix size ({}, {})", 
                    gradient.rows, gradient.cols, 
                    self.weights[layer_index].rows, self.weights[layer_index].cols);
            return;
        }
        
        for i in 0..self.weights[layer_index].rows {
            for j in 0..self.weights[layer_index].cols {
                let current_weight = self.weights[layer_index].get(i, j);
                let gradient_value = gradient.get(i, j);
                let new_weight = self.learning_rate.mul_add(-gradient_value, current_weight);
                self.weights[layer_index].set(i, j, new_weight);
            }
        }
    }
    
    fn update_biases(&mut self, layer_index: usize, gradient: &Vector) {
        let bias_len = self.biases[layer_index].len();
        let gradient_len = gradient.data.len();
        
        for i in 0..bias_len.min(gradient_len) {
            self.biases[layer_index].data[i] -= self.learning_rate * gradient.data[i];
        }
    }
    
    fn train(&mut self, training_data: &[(Vector, Vector)], epochs: usize) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;
            
            for (input, target) in training_data {
                let loss = self.backward(input, target);
                total_loss += loss;
            }
            
            let avg_loss = total_loss / training_data.len() as f64;
            
            if epoch % 100 == 0 {
                println!("Epoch {epoch}: Average Loss = {avg_loss:.6}");
            }
        }
    }
    
    fn predict(&self, input: &Vector) -> Vector {
        let (output, _) = self.forward(input);
        output
    }
}

/// Linear Regression
struct LinearRegression {
    weights: Vector,
    bias: f64,
    learning_rate: f64,
}

impl LinearRegression {
    fn new(input_size: usize, learning_rate: f64) -> Self {
        Self {
            weights: Vector::zeros(input_size),
            bias: 0.0,
            learning_rate,
        }
    }
    
    fn predict(&self, input: &Vector) -> f64 {
        self.weights.dot(input) + self.bias
    }
    
    fn train(&mut self, training_data: &[(Vector, f64)], epochs: usize) {
        for epoch in 0..epochs {
            let mut total_loss = 0.0;
            
            for (input, target) in training_data {
                let prediction = self.predict(input);
                let error = prediction - target;
                
                // Update weights and bias using gradient descent
                for i in 0..self.weights.len() {
                    self.weights.data[i] -= self.learning_rate * error * input.data[i];
                }
                self.bias -= self.learning_rate * error;
                
                total_loss += error * error;
            }
            
            let avg_loss = total_loss / training_data.len() as f64;
            
            if epoch % 100 == 0 {
                println!("Epoch {epoch}: Average Loss = {avg_loss:.6}");
            }
        }
    }
}

/// K-Means Clustering
struct KMeans {
    k: usize,
    centroids: Vec<Vector>,
    max_iterations: usize,
}

impl KMeans {
    const fn new(k: usize, max_iterations: usize) -> Self {
        Self {
            k,
            centroids: Vec::new(),
            max_iterations,
        }
    }
    
    fn initialize_centroids(&mut self, data: &[Vector]) {
        self.centroids.clear();
        
        // Simple initialization: pick random data points
        for _ in 0..self.k {
            let random_index = rand::random::<usize>() % data.len();
            self.centroids.push(data[random_index].clone());
        }
    }
    
    fn assign_clusters(&self, data: &[Vector]) -> Vec<usize> {
        let mut assignments = Vec::new();
        
        for point in data {
            let mut min_distance = f64::INFINITY;
            let mut closest_centroid = 0;
            
            for (i, centroid) in self.centroids.iter().enumerate() {
                let distance = self.euclidean_distance(point, centroid);
                if distance < min_distance {
                    min_distance = distance;
                    closest_centroid = i;
                }
            }
            
            assignments.push(closest_centroid);
        }
        
        assignments
    }
    
    fn update_centroids(&mut self, data: &[Vector], assignments: &[usize]) {
        for k in 0..self.k {
            let cluster_points: Vec<&Vector> = data.iter()
                .zip(assignments.iter())
                .filter(|&(_, &assignment)| assignment == k)
                .map(|(point, _)| point)
                .collect();
            
            if !cluster_points.is_empty() {
                let mut new_centroid = Vector::zeros(data[0].len());
                
                for point in &cluster_points {
                    new_centroid = new_centroid.add(point);
                }
                
                new_centroid = new_centroid.scale(1.0 / cluster_points.len() as f64);
                self.centroids[k] = new_centroid;
            }
        }
    }
    
    fn euclidean_distance(&self, a: &Vector, b: &Vector) -> f64 {
        a.subtract(b).magnitude()
    }
    
    fn fit(&mut self, data: &[Vector]) -> Vec<usize> {
        self.initialize_centroids(data);
        
        for iteration in 0..self.max_iterations {
            let assignments = self.assign_clusters(data);
            let old_centroids = self.centroids.clone();
            
            self.update_centroids(data, &assignments);
            
            // Check for convergence
            let mut converged = true;
            for (old, new) in old_centroids.iter().zip(&self.centroids) {
                if self.euclidean_distance(old, new) > 1e-6 {
                    converged = false;
                    break;
                }
            }
            
            if converged {
                println!("K-Means converged after {} iterations", iteration + 1);
                return assignments;
            }
        }
        
        println!("K-Means reached maximum iterations");
        self.assign_clusters(data)
    }
}

/// Decision Tree Node
#[derive(Debug, Clone)]
enum DecisionNode {
    Leaf { class: String, confidence: f64 },
    Internal {
        feature_index: usize,
        threshold: f64,
        left: Box<DecisionNode>,
        right: Box<DecisionNode>,
    },
}

/// Simple Decision Tree
struct DecisionTree {
    root: Option<DecisionNode>,
    max_depth: usize,
    min_samples_split: usize,
}

impl DecisionTree {
    const fn new(max_depth: usize, min_samples_split: usize) -> Self {
        Self {
            root: None,
            max_depth,
            min_samples_split,
        }
    }
    
    fn fit(&mut self, data: &[Vector], labels: &[String]) {
        self.root = Some(self.build_tree(data, labels, 0));
    }
    
    fn build_tree(&self, data: &[Vector], labels: &[String], depth: usize) -> DecisionNode {
        // Check stopping criteria
        if depth >= self.max_depth || data.len() < self.min_samples_split {
            return self.create_leaf(labels);
        }
        
        // Find best split
        if let Some((feature_index, threshold)) = self.find_best_split(data, labels) {
            let (left_data, left_labels, right_data, right_labels) = 
                self.split_data(data, labels, feature_index, threshold);
            
            if left_data.is_empty() || right_data.is_empty() {
                return self.create_leaf(labels);
            }
            
            let left_node = Box::new(self.build_tree(&left_data, &left_labels, depth + 1));
            let right_node = Box::new(self.build_tree(&right_data, &right_labels, depth + 1));
            
            DecisionNode::Internal {
                feature_index,
                threshold,
                left: left_node,
                right: right_node,
            }
        } else {
            self.create_leaf(labels)
        }
    }
    
    fn create_leaf(&self, labels: &[String]) -> DecisionNode {
        let mut class_counts: HashMap<String, usize> = HashMap::new();
        
        for label in labels {
            *class_counts.entry(label.clone()).or_insert(0) += 1;
        }
        
        let (most_common_class, count) = class_counts.iter()
            .max_by_key(|&(_, &count)| count).map_or_else(|| ("unknown".to_string(), 0), |(class, &count)| (class.clone(), count));
        
        let confidence = count as f64 / labels.len() as f64;
        
        DecisionNode::Leaf {
            class: most_common_class,
            confidence,
        }
    }
    
    fn find_best_split(&self, data: &[Vector], labels: &[String]) -> Option<(usize, f64)> {
        let mut best_gini = f64::INFINITY;
        let mut best_split = None;
        
        for feature_index in 0..data[0].len() {
            let mut values: Vec<f64> = data.iter().map(|v| v.data[feature_index]).collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            values.dedup();
            
            for i in 0..values.len() - 1 {
                let threshold = f64::midpoint(values[i], values[i + 1]);
                let gini = self.calculate_gini_impurity(data, labels, feature_index, threshold);
                
                if gini < best_gini {
                    best_gini = gini;
                    best_split = Some((feature_index, threshold));
                }
            }
        }
        
        best_split
    }
    
    fn calculate_gini_impurity(&self, data: &[Vector], labels: &[String], feature_index: usize, threshold: f64) -> f64 {
        let (left_data, left_labels, right_data, right_labels) = 
            self.split_data(data, labels, feature_index, threshold);
        
        let total_samples = data.len() as f64;
        let left_weight = left_data.len() as f64 / total_samples;
        let right_weight = right_data.len() as f64 / total_samples;
        
        let left_gini = self.gini_impurity(&left_labels);
        let right_gini = self.gini_impurity(&right_labels);
        
        left_weight.mul_add(left_gini, right_weight * right_gini)
    }
    
    fn gini_impurity(&self, labels: &[String]) -> f64 {
        if labels.is_empty() {
            return 0.0;
        }
        
        let mut class_counts: HashMap<String, usize> = HashMap::new();
        for label in labels {
            *class_counts.entry(label.clone()).or_insert(0) += 1;
        }
        
        let total = labels.len() as f64;
        let mut gini = 1.0;
        
        for count in class_counts.values() {
            let probability = *count as f64 / total;
            gini -= probability * probability;
        }
        
        gini
    }
    
    fn split_data(&self, data: &[Vector], labels: &[String], feature_index: usize, threshold: f64) 
        -> (Vec<Vector>, Vec<String>, Vec<Vector>, Vec<String>) {
        let mut left_data = Vec::new();
        let mut left_labels = Vec::new();
        let mut right_data = Vec::new();
        let mut right_labels = Vec::new();
        
        for (i, sample) in data.iter().enumerate() {
            if sample.data[feature_index] <= threshold {
                left_data.push(sample.clone());
                left_labels.push(labels[i].clone());
            } else {
                right_data.push(sample.clone());
                right_labels.push(labels[i].clone());
            }
        }
        
        (left_data, left_labels, right_data, right_labels)
    }
    
    fn predict(&self, input: &Vector) -> Option<String> {
        self.root.as_ref().map(|root| self.predict_node(root, input))
    }
    
    fn predict_node(&self, node: &DecisionNode, input: &Vector) -> String {
        match node {
            DecisionNode::Leaf { class, .. } => class.clone(),
            DecisionNode::Internal { feature_index, threshold, left, right } => {
                if input.data[*feature_index] <= *threshold {
                    self.predict_node(left, input)
                } else {
                    self.predict_node(right, input)
                }
            }
        }
    }
}

/// สาธิตการใช้งาน Machine Learning
pub fn demonstrate_machine_learning() {
    println!("🤖 Machine Learning and AI Examples:");
    
    // Linear Regression
    println!("\n📈 Linear Regression:");
    println!("{:-<50}", "");
    
    let mut linear_model = LinearRegression::new(1, 0.01);
    
    // Training data: y = 2x + 1 + noise
    let training_data = vec![
        (Vector::new(vec![1.0]), 3.1),
        (Vector::new(vec![2.0]), 5.2),
        (Vector::new(vec![3.0]), 6.9),
        (Vector::new(vec![4.0]), 9.1),
        (Vector::new(vec![5.0]), 10.8),
    ];
    
    println!("Training linear regression model...");
    linear_model.train(&training_data, 1000);
    
    println!("\nPredictions:");
    for (input, actual) in &training_data {
        let prediction = linear_model.predict(input);
        println!("Input: {:.1}, Actual: {:.1}, Predicted: {:.2}", 
                input.data[0], actual, prediction);
    }
    
    // Neural Network
    println!("\n🧠 Neural Network:");
    println!("{:-<50}", "");
    
    let mut nn = NeuralNetwork::new(&[2, 3, 1], 0.1);
    
    // XOR problem
    let nn_training_data = vec![
        (Vector::new(vec![0.0, 0.0]), Vector::new(vec![0.0])),
        (Vector::new(vec![0.0, 1.0]), Vector::new(vec![1.0])),
        (Vector::new(vec![1.0, 0.0]), Vector::new(vec![1.0])),
        (Vector::new(vec![1.0, 1.0]), Vector::new(vec![0.0])),
    ];
    
    println!("Training neural network for XOR problem...");
    nn.train(&nn_training_data, 1000);
    
    println!("\nXOR Predictions:");
    for (input, target) in &nn_training_data {
        let prediction = nn.predict(input);
        println!("Input: [{:.0}, {:.0}], Target: {:.0}, Predicted: {:.3}", 
                input.data[0], input.data[1], target.data[0], prediction.data[0]);
    }
    
    // K-Means Clustering
    println!("\n🎯 K-Means Clustering:");
    println!("{:-<50}", "");
    
    let clustering_data = vec![
        Vector::new(vec![1.0, 1.0]),
        Vector::new(vec![1.5, 2.0]),
        Vector::new(vec![3.0, 4.0]),
        Vector::new(vec![5.0, 7.0]),
        Vector::new(vec![3.5, 5.0]),
        Vector::new(vec![4.5, 5.0]),
        Vector::new(vec![3.5, 4.5]),
    ];
    
    let mut kmeans = KMeans::new(2, 100);
    let clusters = kmeans.fit(&clustering_data);
    
    println!("\nClustering Results:");
    for (i, (point, cluster)) in clustering_data.iter().zip(&clusters).enumerate() {
        println!("Point {}: [{:.1}, {:.1}] -> Cluster {}", 
                i, point.data[0], point.data[1], cluster);
    }
    
    println!("\nCentroids:");
    for (i, centroid) in kmeans.centroids.iter().enumerate() {
        println!("Cluster {}: [{:.2}, {:.2}]", 
                i, centroid.data[0], centroid.data[1]);
    }
    
    // Decision Tree
    println!("\n🌳 Decision Tree:");
    println!("{:-<50}", "");
    
    let tree_data = vec![
        Vector::new(vec![5.1, 3.5]),  // setosa
        Vector::new(vec![4.9, 3.0]),  // setosa
        Vector::new(vec![7.0, 3.2]),  // versicolor
        Vector::new(vec![6.4, 3.2]),  // versicolor
        Vector::new(vec![6.3, 3.3]),  // versicolor
        Vector::new(vec![6.5, 2.8]),  // virginica
        Vector::new(vec![7.6, 3.0]),  // virginica
    ];
    
    let tree_labels = vec![
        "setosa".to_string(),
        "setosa".to_string(),
        "versicolor".to_string(),
        "versicolor".to_string(),
        "versicolor".to_string(),
        "virginica".to_string(),
        "virginica".to_string(),
    ];
    
    let mut decision_tree = DecisionTree::new(3, 2);
    decision_tree.fit(&tree_data, &tree_labels);
    
    println!("Decision Tree Predictions:");
    for (i, sample) in tree_data.iter().enumerate() {
        if let Some(prediction) = decision_tree.predict(sample) {
            println!("Sample {}: [{:.1}, {:.1}] -> Predicted: {}, Actual: {}", 
                    i, sample.data[0], sample.data[1], prediction, tree_labels[i]);
        }
    }
    
    // Activation Functions Demo
    println!("\n⚡ Activation Functions:");
    println!("{:-<50}", "");
    
    let test_values = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    
    println!("Input\tSigmoid\tReLU\tTanh");
    println!("{:-<40}", "");
    
    for &x in &test_values {
        println!("{:.1}\t{:.3}\t{:.3}\t{:.3}", 
                x, 
                ActivationFunctions::sigmoid(x),
                ActivationFunctions::relu(x),
                ActivationFunctions::tanh(x));
    }
    
    // Softmax Demo
    println!("\n🎲 Softmax Function:");
    println!("{:-<50}", "");
    
    let logits = vec![2.0, 1.0, 0.1];
    let probabilities = ActivationFunctions::softmax(&logits);
    
    println!("Logits: {logits:?}");
    println!("Probabilities: {probabilities:?}");
    println!("Sum: {:.6}", probabilities.iter().sum::<f64>());
    
    // Loss Functions Demo
    println!("\n📊 Loss Functions:");
    println!("{:-<50}", "");
    
    let predicted = vec![0.9, 0.8, 0.1];
    let actual = vec![1.0, 1.0, 0.0];
    
    let mse = LossFunctions::mean_squared_error(&predicted, &actual);
    let cross_entropy = LossFunctions::cross_entropy(&predicted, &actual);
    
    println!("Predicted: {predicted:?}");
    println!("Actual: {actual:?}");
    println!("Mean Squared Error: {mse:.6}");
    println!("Cross Entropy: {cross_entropy:.6}");
    
    println!("\n✅ Machine learning examples demonstrated!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_operations() {
        let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
        let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
        
        assert_eq!(v1.dot(&v2), 32.0);
        assert_eq!(v1.add(&v2).data, vec![5.0, 7.0, 9.0]);
        assert_eq!(v1.scale(2.0).data, vec![2.0, 4.0, 6.0]);
    }
    
    #[test]
    fn test_matrix_operations() {
        let m1 = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let m2 = Matrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
        
        let result = m1.multiply(&m2);
        assert_eq!(result.data, vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
    }
    
    #[test]
    fn test_activation_functions() {
        assert!((ActivationFunctions::sigmoid(0.0) - 0.5).abs() < 1e-10);
        assert_eq!(ActivationFunctions::relu(-1.0), 0.0);
        assert_eq!(ActivationFunctions::relu(1.0), 1.0);
        assert!((ActivationFunctions::tanh(0.0) - 0.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_loss_functions() {
        let predicted = vec![0.5, 0.8];
        let actual = vec![1.0, 1.0];
        
        let mse = LossFunctions::mean_squared_error(&predicted, &actual);
        assert!((mse - 0.145).abs() < 1e-3);
    }
}