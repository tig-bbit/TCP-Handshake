# tcp_handshake

I have written this script in which we will run a p2p network using substrate node template.
---------------------------------
#**Steps**

---------------------------------
##Setup RUST
First we need to install rust in our system by running the following command.

**curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh**
 
To verify that Rust is installed run following command

**rustc --version**

---------------------------------
##Setup a p2p network.

lets follow the command one by one

**git clone git@github.com:Rusted2361/tcp_handshake.git**

**cd substrate-node-template**

**cargo build --release**

**./target/release/node-template --dev**

---------------------------------
##Test Local p2p nodes

The system will serve at local host on port 9944. This will start a local p2p dev network on your system that you can access on this link.

**https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer**

After  setting up environment & running p2p network we need to test or tcp_handshake script.

---------------------------------
##Perform Handshake with p2p system
we will simply go to the root directory and run command

**cargo run 
**

It will show you following message.

Substrate handshake successful. Remote version: 791752241