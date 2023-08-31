# Prime Number Generator

This Rust program generates prime numbers within a given range .

## Usage

1. Install Rust: Make sure you have Rust installed on your system. If not, you can install it from the official Rust website: https://www.rust-lang.org/tools/install

2. Clone the Repository: Clone this repository to your local machine using the following command:

   ```bash
   git clone https://github.com/your-username/prime-number-generator.git
# now go to the directory
cd prime-number-generator
# run the rust programme
cargo run
# Now build the .wasm file using the command:
cargo build --release --target wasm32-wasi
# to dockerise the program run 
docker build .
# now run the program in the container
docker run -it image_name


