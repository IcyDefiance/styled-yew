# styled-yew

CSS in Rust, similar to styled-components, but for Yew.

## Syntax

    styled!(pub RedDiv : Div {
        color: "red";
    });

See the example [here](https://github.com/IcyDefiance/styled-yew/blob/master/example/src/lib.rs).

## Running the Example

    cargo install wasm-pack
    cd example
    npm i
    npm start -- --open
