# transfer_cspr

Install contract.

```bash
casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 90000000000 \
--session-path <work dir>/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--secret-key <deployer_key>/secret_key.pem 
```

Deploy `transfer_cspr` . 

They deploy key should be user's key
```bash
casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 20000000000 \
--session-path <work dir>/contract/target/wasm32-unknown-unknown/release/transfer_cspr.wasm \
--session-entry-point transfer_cspr \
--session-arg "amount:U512='330000000'" \
--session-arg "contract_hash:Key='hash-xxx'" \
--session-arg "target:Key='account-hash-xxx'" \
--secret-key <user_key>/secret_key.pem 
```