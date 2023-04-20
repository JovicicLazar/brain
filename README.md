# brain

brain - is a simple brainfuck interpreter. It takes in a .bf file and runs it instruction by istruction.
This version doesnt  have 30000 memory slots like original compiler has, instead data is saved on a hash map, which size
depends on your machines memory. Also this version has memory wrapping capability.

## Requirements
- Rust 

## Installation
1. Clone the repository
2. Compile the code
3. Run the code

## Compile:
````commandline
rustc main.rs -o brain.exe => for windows
rustc main.rs -o brain => for linux
````

## Run:

```commandline
brain file_name.rs
``` 
Feel free to contact me: lazarjovicic808@gmail.com
