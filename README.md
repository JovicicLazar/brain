# brain - a brainfuck compiler

brain - is a user-friendly brainfuck interpreter designed to execute code from a .bf file, one instruction at a time. Unlike the original compiler, which had a fixed limit of 30,000 memory slots, "brain" stores data in a hash map that dynamically adjusts to your machine's memory size. Additionally, "brain" has a memory wrapping feature that allows for seamless execution of code that exceeds the memory capacity. Overall, "brain" is a simple tool for running brainfuck code on your machine.

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
brain file_name.bf
``` 
I will add some examples in the future :)

Feel free to contact me: lazarjovicic808@gmail.com
