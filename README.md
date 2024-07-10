# Multiple choice test simulator
The Multiple Choice Test Simulator is a simple quiz application designed to assist users in creating multiple-choice quizzes for studying purposes.

It is developed using the Rust programming language and the egui framework, enabling compilation as both a desktop and web application.


# Build
## Build as desktop app
To run it as a desktop application simply build it as any other Rust application
```
cargo build
```

## Build for web
For web deployment you will need to install [wasm-pack](https://rustwasm.github.io/wasm-pack/)

Than simply run the following command
```
wasm-pack build --no-typescript -t web
```

Finally you will need to host `index.html` file and entiker `pkg` file (where the wasm module is stored)