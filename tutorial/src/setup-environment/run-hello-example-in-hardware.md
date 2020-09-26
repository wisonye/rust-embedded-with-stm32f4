#### <a name="run-hello-example-in-hardware">3.4 Run hello example in hardware<a>

- First, make sure you go through this [step](https://rust-embedded.github.io/book/intro/install/verify.html) to 
make sure all hardware connections already work.

- We use `--target` in `cargo` command, then we don't need to modify `.cargo/config` again

    ```bash
    cargo run --target thumbv7em-none-eabihf --example hello
    ```

- For debugging, plz follow steps [here](https://rust-embedded.github.io/discovery/05-led-roulette/flash-it.html)
