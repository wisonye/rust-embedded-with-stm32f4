# All-in-one Rust Embedded Programming for STM32F4 series
In this book, we will cover all the pieces of knowledge we need to know for using `Rust` to do embedded programming.

And we pick the `ARM-based MCU STM32F4` series chips as our target to run all the demos.

## About the book in `tutorial` folder

- The book is created by [`mdbook`](https://rust-lang.github.io/mdBook/).

- How to install `mdBook`

    - Install via `cargo`:
        
        ```bash
        cargo install mdBook
        ```
    - About another way to install, plz check [here](https://github.com/rust-lang/mdBook)

- How to view the book in your browser

    Make sure you're in the repo root folder and run:

    ```bash
    # Clean the prev build
    mdbook clean ./tutorial

    # Serve it via HTTP server
    mdbook serve --open ./tutorial
    ```

    It will build the book into `tutorial/book` folder and open it your browser.

- Or you can open the PDF version [here](./all-in-one_rust_embedded_programming_for_stm32f4_series.pdf)

