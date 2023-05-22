# clipboard-typer
A small windows program that can type out the current content of the clipboard.
Simply start the program and press `insert` on your keyboard to start typing.
Linebreaks are converted to `enter` presses, so the result might be a bit messed up if `enter` does not place the cursor at the beginning of the new line. 

## Running

```bash
git clone https://github.com/sidit77/clipboard-typer.git
cd clipboard-typer
cargo run --release
```

## License
MIT License