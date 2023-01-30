/*A stress test that hammers the CPU and GPU using PyTorch
Use this as a guide to work from:
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let device = match a.iter().map(|x| x.as_str()).collect::<Vec<_>>().as_slice() {
        [_] => tch::Device::Cpu,
        [_, "cpu"] => tch::Device::Cpu,
        [_, "gpu"] => tch::Device::Cuda(0),
        _ => panic!("usage: main cpu|gpu"),
    };
    let slice = vec![0; 1_000_000];
    for i in 1..1_000_000 {
        let t = Tensor::of_slice(&slice).to_device(device);
        println!("{} {:?}", i, t.size())
    }
}
*/
use tch::Tensor;

//build a cpu load test function
pub fn cpu_load_test() {
    let slice = vec![0; 1_000_000];
    for i in 1..1_000_000 {
        let t = Tensor::of_slice(&slice).to_device(tch::Device::Cpu);
        println!("{} {:?}", i, t.size())
    }
}

//build a gpu load test function
pub fn gpu_load_test() {
    let slice = vec![0; 1_000_000];
    for i in 1..1_000_000 {
        let t = Tensor::of_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}", i, t.size())
    }
}
