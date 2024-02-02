# BitVM.rs

Experimental BitVM implementation in Rust.

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)
to verify the trustworthiness of each of your dependencies, including this one.


```
bitcoind -regtest -rpcuser=admin -rpcpassword=admin -rpcport=18443 -fallbackfee=0.00001 -wallet=admin
```

```
bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin createwallet "admin"
```

```
bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin generatetoaddress 101 $(bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin getnewaddress)
```
