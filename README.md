### PyTorch Rust GPU Example

A repo to show how GPUs work with Rust and PyTorch.
`export TORCH_CUDA_VERSION=cu117`

The cd into `pytorch-gpu-util` and run `cargo run -- gpu`

### Debugging Bindings

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