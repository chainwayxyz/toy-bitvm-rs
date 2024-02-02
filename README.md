# Toy BitVM in Rust

Experimental toy BitVM implementation in Rust.

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)
to verify the trustworthiness of each of your dependencies, including this one.

Run regtest with the following command:
```
bitcoind -regtest -rpcuser=admin -rpcpassword=admin -rpcport=18443 -fallbackfee=0.00001 -wallet=admin
```

Then run the following command to generate blocks continuously:
```
./regtest-commands.sh
```

Then start the verifier binary with the following command:
```
cargo run --bin verifier
```

Start the prover binary with the following command:
```
cargo run --bin prover
```

From now on, you can start challenging gates and waiting for the prover to respond. 
There is a fraud hardcoded in the code. Challenge `64` for first, then `63` to see the fraud and slash the prover.
