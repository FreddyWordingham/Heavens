# Constellation

<div align="center">
    <img src="./images/logo.svg" alt="Galaxy simulation" width=200>
</div>

N-body simulation.

## Quickstart

Clone the repository and change into the directory:

```shell
git clone https://github.com/FreddyWordingham/Constellation
cd Constellation
```

Compile the code:

```shell
cargo run --release -- --radius 1e6 --res 64 --grav-strength 1.0e2 --smoothing-length 1e3 --num-stars 1000
```

![Terminal simulation](./images/screenshot.png)
