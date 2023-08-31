# docker_wasm
In this repository i have dockerised  wasm compiled rust program to calculate the 47th fibonacci number 

# first run the programme

cargo run main.rs

# now build this program to wasm run :
cargo build --release --target wasm32-wasi
# now we can run the .wasm file using any of the wasm runtimes such as wasmer , wasmedge , wasmtime , wasm3 etc

to run with wasmer :

wasmer filename.wasm
# to dockerize the program create Dockerfile and run :

docker build .
# to run the dockerfile run:
docker run -it image_name_or_id
