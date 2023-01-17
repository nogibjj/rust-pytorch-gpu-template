### PyTorch Rust GPU Example

Exploration of these EXCELLENT bindings of PyTorch and Rust: https://github.com/LaurentMazare

#### Hello World Stress Test
A repo to show how GPUs work with Rust and PyTorch.
`export TORCH_CUDA_VERSION=cu117`

The cd into `pytorch-gpu-util` and run `cargo run -- gpu`


One tip is to look into your build to ensure the crate actually downloaded the cuda version:
```bash
 ls -l /workspaces/rust-pytorch-gpu-template/pytorch-gpu-util/target/debug/build/torch-sys-0893541a21a2091d/out/libtorch/libtorch/lib | grep cuda
-rw-rw-rw- 1 codespace codespace   1235152 Jan 16 22:18 libc10_cuda.so
-rw-rw-rw- 1 codespace codespace    828800 Jan 16 22:18 libc10d_cuda_test.so
-rw-rw-rw- 1 codespace codespace    687320 Jan 16 22:20 libcudart-e409450e.so.11.0
-rw-rw-rw- 1 codespace codespace   7221084 Jan 16 22:18 libgloo_cuda.a
-rw-rw-rw- 1 codespace codespace   3769170 Jan 16 22:18 libtensorpipe_cuda.a
-rw-rw-rw- 1 codespace codespace 382610744 Jan 16 22:19 libtorch_cuda_cpp.so
-rw-rw-rw- 1 codespace codespace 753941192 Jan 16 22:20 libtorch_cuda_cu.so
-rw-rw-rw- 1 codespace codespace 219665888 Jan 16 22:20 libtorch_cuda_linalg.so
-rw-rw-rw- 1 codespace codespace      7496 Jan 16 22:20 libtorch_cuda.so
```

#### MNIST Convolutional Neural-Network

Ensure this variable is set: `export TORCH_CUDA_VERSION=cu117`
cd into `pytorch-mnist` and run `cargo run -- conv`.

#### Stable diffusion demo

* clone this repo:  https://github.com/LaurentMazare/diffusers-rs
* Follow these setup instructions: https://github.com/LaurentMazare/diffusers-rs#clip-encoding-weights

After all the weights are downloaded run:

`cargo run --example stable-diffusion --features clap -- --prompt "A very rusty robot holding a fire torch to notebooks"`
![Screenshot 2023-01-16 at 5 57 59 PM](https://user-images.githubusercontent.com/58792/212777548-0d9619e8-ad1b-4cc9-8871-505b0b5b2345.png)
