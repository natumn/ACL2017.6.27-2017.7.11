extern crate serde_pickle;
extern crate ndarray::linalg;
use ndarray::prelude::*;


fn get_data() -> (x_test: Vec<f64>,t_test:  Vec<f64>) {
    let x_train,t_train,x_test,t_test = load_mnist(true, true, false);
}

//TODO: Reading weight parameter saved sample_weight.pkl
fn init_network() -> Vec<f64> {
    let network = pickle::value_from_reader("/sample_weight.pkl");
}

//TODO: network build
fn predict(network: Vec<f64>, x: Vec<f64>) -> Vec<f64> {
    //network array has three weight (index: 0..2) and three bias (index: 3..5)

    let a1 = linalg::Dot::dot(x, network[0]) + network[3];
    let z1 = sigmoid(a1);
    let a2 = linalg::Dot::dot(z1, network[1]) + network[4];
    let z2 = sigmoid(a2);
    let a3 = linalg::Dot::dot(z2, network[2]) + network[5];
    let y = softmax(a3);
}


fn softmax(x: f64) -> f64 {
    
}

fn sigmoid(x: f64) -> f64 {
    let e = 2.718;
    1/(1 + e.pow(-x))
}

fn main() {
    let x, t = get_data();
    let mut network = init_network();
    let mut accuracy_cnt = 0;

    for (i,x) in x.enumerate() {
        let p = predict(network, x).fold(0.0/0.0, f64::max);
        if p == t[i] {
            accracy_cut += 1;
        }
    }
    println!("Accuracy: {}" accuracy_cut/x.len());
}
