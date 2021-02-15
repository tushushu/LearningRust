# LearningRust
Learning Rust by `<The Rust Programming Language>`.  
The Book link is: https://doc.rust-lang.org/book/  

## Debugging Rust in VSCode  
1. Install VSCode;
2. Install Rust by `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`;
3. Configure your shell by `source $HOME/.cargo/env`;
4. Update Rust by `rustup update`;
5. Run `rustup component add rls-preview`;
6. Run `rustup component add rust-analysis --toolchain stable`;
7. Run `rustup component add rust-src --toolchain stable`;
8. Intall VSCode extensions named `rust-analyzer` and `CodeLLDB`, and do not install `Rust` because it has conflicts with `rust-analyzer`.


