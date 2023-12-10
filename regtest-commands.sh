#!/bin/bash

bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin createwallet "admin";

bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin generatetoaddress 101 $(bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin getnewaddress);

while true
do
    bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin generatetoaddress 1 $(bitcoin-cli -regtest -rpcuser=admin -rpcpassword=admin getnewaddress)
    sleep 1
done