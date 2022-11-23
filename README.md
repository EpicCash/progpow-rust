# ProgPoW - Rust

ProgPoW is a proof-of-work algorithm designed to close the efficiency gap available to specialized ASICs. It utilizes almost all parts of commodity hardware (GPUs), and comes pre-tuned for the most common hardware.

This is a rust build of the current C++ algorithm.

## Build steps

To build the project you will have to specify if the application is `only CPU`, `OPENCL`, or `CUDA`.
To build for an application using **CPU**, execute the following line in the terminal:

```sh
cargo build
```

To build for an application using **CPUs/GPUs** use `OPENCL`. Execute the following line in the terminal to build with `OPENCL`:

```sh
cargo build --features opencl
```

If you have NVIDIA GPUs and your system has **the latest Nvidia drivers and the Cuda toolkit 9+ installed**, you can build the Cuda plugins using the following command:

```sh
cargo build --no-default-features --features cuda
```

## What was built

The rust library of the ProgPoW algorithm