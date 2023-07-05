# trade_nft

## Prerequisite 
Install NFT (cep47 NFT for this example)


## STEP1: Install contract.

```bash
casper-client put-deploy \
--chain-name casper-net-1 \
--node-address http://localhost:11101 \
--payment-amount 150000000000 \
--session-path /home/jh/mywork/transfer_contract/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--session-arg "nft_contract:Key='hash-842af5d9424fe20700711facbd30e45d313c26e8981b4ccf38696e36af559d81'" \
--secret-key /home/jh/casper-node/utils/nctl/assets/net-1/users/user-1/secret_key.pem

```

## STEP2: Deploy `trade_nft` session contract. 

```bash
casper-client put-deploy \
--chain-name casper-net-1 \
--node-address http://localhost:11101 \
--payment-amount 50000000000 \
--session-path /home/jh/mywork/transfer_contract/contract/target/wasm32-unknown-unknown/release/trade_nft.wasm \
--session-arg "amount:U512='330000000'" \
--session-arg "contract_hash:Key='hash-6bec40ac1681e785196307c711ad0e2ab5934ac984cfacb841142136a3a07c4c'" \
--session-arg "target:Key='account-hash-7b8f3db6c0e08eb7c083547d6ef7b15c0d3615ef71d54d500b5fec31939ac747'" \
--secret-key /home/jh/casper-node/utils/nctl/assets/net-1/users/user-1/secret_key.pem
```
note:
contract_hash is from STEP1