# Compilation & Running
- The workflow is almost like C. To compile a rust file:
```
rustc ./hellow-world.rs 
```
- Then run it like normal executables:
```
./hello-world
```

# Cargo
Cargo is Rust’s build system and package manager.

You can bootstrap a Rust project with Cargo:
```
cargo new hello-rust
```
Then in the newly created folder, run this to compile the files (default will go to a `/debug` folder):
```
cargo build
```
Or run this compile AND run the code:
```
cargo run
```
To compile to a production build with optimisations:
```
cargo build --release
```