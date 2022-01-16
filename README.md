<!-- ABOUT THE PROJECT -->
## About The Project

This project implements the encoder/decoder of GOOSE(Generic Object Oriented Substation Event) protocol defined in IEC61850, with Rust.


<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

* Rust (>1.5)
    https://www.rust-lang.org/learn/get-started
    

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/orbitdoc/goose_packet.git
   ```

2. Build Rust packages
   ```sh
   cd goose_packet/
   cargo build
   ```

<!-- USAGE EXAMPLES -->
## Usage
There are multile examples in src/bin. 

1. Example of encoding and decoding GOOSE frames:
   ```sh
   cargo run --bin example_encode_decode
   ```

2. Example of sending a GOOSE frame:
   ```sh
   cargo run --bin example_tx 'name-of-your-network-interface' 
   ```
   Replace `'name-of-your-network-interface'` with the network interface to be used for sending GOOSE frame.
   In Windows, it may be `'\Device\NPF_{?????????}' `; In Linux, it may be `'eth0'`.
   You can also try:

   ```sh
   cargo run --bin example_tx
   ```  
   The error message will list all available network interfaces.
    Notice, `sudo` may be needed to run the example in Linux:

    ```sh
    sudo ./target/debug/example_tx 'name-of-your-network-interface'
    ```  

3. Example of receiving GOOSE frame(s):

    In fisrt teminal, run the following example to start listening GOOOSE packets. 
   ```sh
   cargo run --bin example_rx 'name-of-your-network-interface' 
   ```
    In the second terminal, run example 2, and there should be update in the first terminal.

    Again, when `sudo` is needed, try:
     ```sh
    sudo ./target/debug/example_rx 'name-of-your-network-interface'
    ```     