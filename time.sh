cd ./rust
cargo build --release
echo "Starting Rust"
time ./target/release/anagram
echo -e "\n\n-----------------"

cd ..

cd python
echo "Starting Python"
time python3 main.py
echo -e "\n\n-----------------"
