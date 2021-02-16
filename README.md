# LearningRust
Learning Rust by `<The Rust Programming Language>`.  
The Book link is: https://doc.rust-lang.org/book/  

## Debugging Rust in VSCode  
  * Install VSCode;
  * Install Rust by `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`;
  * Configure your shell by `source $HOME/.cargo/env`;
  * Update Rust by `rustup update`;
  * Run `rustup component add rls-preview`;
  * Run `rustup component add rust-analysis --toolchain stable`;
  * Run `rustup component add rust-src --toolchain stable`;
  * Intall VSCode extensions named `Rust` and `CodeLLDB`.

## Useful commands
  * `pkill rls cargo` to stop cargo building tasks;
  * `rm -rf ~/.cargo/registry/index/*` to remove compiler lock file;
  * `shift + command + p`, then `rls.stop` to stop RLS.
