Prover proves that:
- gave every withdrawal requests BTC from their own capital.
- moved remaining capital to the next BitVM instance.

Verifier should be able to slash the prover if:
Prover didn't make the specified tasks in 6 days


If prover fails to give the prove by the time:
- lost his stake
- lost the connector output so can't steal bridge money.

Make a bitcoin transaction

UTXO 1:
Paul's Stake, 10 BTC

UTXO 2:
Vicky's Stake, 1 BTC

3 Outputs:
Output 1:
Vicky's Challenge Root, 0 BTC

Output 2:
11 BTC
2-of-2 Multisig
or
Paul Takes in 7 Days
or
Vicky Slashes equivocation

Output 3:
0 BTC
Connector output
2-of-2 Multisig
Paul Takes in 8 Days (7 Days + challenge response protocol)

Paul also gives signatures for spending connector output for every step at challenge response protocol.

Depositor puts money to the 2-of-2 multisig by the prover and depositor
Gives 2*(Total number of rounds) signatures:
- spend the deposit with connector output to the provers address
- spend the deposit to the next 2-of-2 multisig.
- spend the second multisig with second connector output to the provers address
- spend the seocond multisig to the next 2-of-2 multisig
...


Questions:
Can SIGHASH_ALL | SIGHASH_ANYONECANPAY be used?
Can we add delayed upgradability?