# untangle-rs

A game written in Rust for the [WASM-4](https://wasm4.org) fantasy console.

## How to play

You are presented with a collection of points on a plane, and some of these points are connected by straight lines. The challenge is to manipulate the positions of these points freely, with the ultimate objective of repositioning them in such a way that no line segment crosses another.

**Controls**: Select a point by tapping or clicking on it, then drag it to a new position.

## Building

Build the game by running:

```shell
cargo build --release
```

Then run it with:

```shell
cd target/wasm32-unknown-unknown/release
w4 run untangle.wasm
```

You can optimize the binary file size using `wasm-opt` or `wasm-snip`, and then bundle it into an HTML file using the `w4 bundle` subcommand. Please ensure that you have the necessary tools installed and that your environment is properly set up to execute these commands.

```shell
# In target/wasm32-unknown-unknown/release
wasm-opt -Oz untangle.wasm -o untangle.wasm
wasm-snip untangle.wasm -o untangle.wasm env::memory
w4 bundle untangle.wasm --title "untangles" --html untangle.html
```

For more info about setting up WASM-4, see the [quickstart guide](https://wasm4.org/docs/getting-started/setup?code-lang=rust#quickstart).

## Links

- [Documentation](https://wasm4.org/docs): Learn more about WASM-4.
- [Snake Tutorial](https://wasm4.org/docs/tutorials/snake/goal): Learn how to build a complete game
  with a step-by-step tutorial.
- [GitHub](https://github.com/aduros/wasm4): Submit an issue or PR. Contributions are welcome!
