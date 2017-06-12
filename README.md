# Conway's Game of Life

This code is an implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life), written in Rust.

To visualise, run the code

```sh
cargo run --release -- --width 100 --height 100 --niter 100 --output gol.txt
```

and visualise with `plot_gol.py`:

```sh
python ./plot_gol.py gol.txt -o gol.mp4
open gol.mp4
```
