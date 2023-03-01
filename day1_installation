## Installation

### Installing rustup on Linux or macOS

'''
$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
'''

Once you saw 'Rust is installed now. Great!' means you Rust has been successfully installed

Can use command rustc --version to check your current version, you should see something like this -> rustc 1.67.1 (d5a82bbd2 2023-02-07)

### Updating and Uninstalling
Once Rust is installed via rustup, updating to a newly released version is easy. From your shell, run the following update script:
$ rustup update

To uninstall Rust and rustup, run the following uninstall script from your shell:
$ rustup self uninstall

### Your first rust programe
Create a file named main.rs and input below function. Using command rustc main.rs to compile the file first.
Then use command ./main.rs to run the program and you should see Hello world has been print out in the console.

### Something need to be noticed
- There is a '!' after println which indicates this is not a function is a Rust Macro
- And unlike Python, similar to Java there is a semicolon after each line

'''
fn main() {
    println!("Hello world");
}
'''

### Hello Cargo
Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

Use -> cargo --version to check current version
Use -> cargo new [project name] to create a new cargo project
Use -> cargo build to build & compile your Rust programe, file will be build under ./target/debug/
Use -> cargo run to complie and run your project in one go
Use -> cargo check to check your code, make sure it can complies(won't produce any executable file)
Use -> cargo build --release to build a released version binary executable file




