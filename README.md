# sorobanathon-allowance

1. Build the wasm
  ```
  cargo build --target wasm32-unknown-unknown --release
  ```

2. Run the future net node
  ```
  docker run --rm -it \
    --platform linux/amd64 \
    -p 8000:8000 \
    --name stellar \
    stellar/quickstart:soroban-dev@sha256:8046391718f8e58b2b88b9c379abda3587bb874689fa09b2ed4871a764ebda27 \
    --futurenet \
    --enable-soroban-rpc
  ```

3. Make sure it's running with a `core_latest_ledger` larger than 0
  ```
  curl http://localhost:8000
  ```

4. Generate key pair
  ```
  Public Key	GDT3C2AAQUXC2Y3V6HFVG3BXWG3P6OGTYLJTNI7YED6AU6NZLW2AOQY3
  Secret Key	SB3O7LX4CJO4Z2WCT7RZQ7RZTFUPDV6COAAO4E3LMPQ32NM24G3D72D2
  ```

5. Fund the wallet
  ```
  curl "https://friendbot-futurenet.stellar.org/?addr=G..."
  curl "https://friendbot-futurenet.stellar.org/?addr=GDT3C2AAQUXC2Y3V6HFVG3BXWG3P6OGTYLJTNI7YED6AU6NZLW2AOQY3"
  ```

6. Deploy the contract
  ```
  soroban deploy \
    --wasm parent_allowance.wasm \
    --secret-key SB3O7LX4CJO4Z2WCT7RZQ7RZTFUPDV6COAAO4E3LMPQ32NM24G3D72D2 \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022'
  ```
  Contract ID: 07cc3fbe8350d9f4135b6065f3dc9599a12d58b1266223ff7cc259c429008a54

7. Deploy token contract
  ```
  soroban deploy \
    --wasm soroban_token_contract.wasm \
    --secret-key SB3O7LX4CJO4Z2WCT7RZQ7RZTFUPDV6COAAO4E3LMPQ32NM24G3D72D2 \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022'
  ```
  Contract ID: 8216da0fc28346cdd27a61819f93cc7c1488584717ba6e94a8e45a5daa084688

8. Invoke Token contract initialization
  ```
  soroban invoke \
    --id 8216da0fc28346cdd27a61819f93cc7c1488584717ba6e94a8e45a5daa084688 \
    --secret-key SB3O7LX4CJO4Z2WCT7RZQ7RZTFUPDV6COAAO4E3LMPQ32NM24G3D72D2 \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn initialize \
    --arg '{"object":{"vec":[{"symbol":"Account"},{"object":{"account_id":{"public_key_type_ed25519":"e7b16800852e2d6375f1cb536c37b1b6ff38d3c2d336a3f820fc0a79b95db407"}}}]}}' \
    --arg 8 \
    --arg 44616e69656c \
    --arg 444e44
  ```
  Name: Daniel<br>
  Symbol: DND

9. Create a parent account
  ```
  Public Key	GBWJIH5VDCIKAVFOJK4N5RH5O7NZ27R3V6OSJGJ4JUQO5XB6U73CXYNL
  Secret Key	SDVSMGYKIFR3MLUPY5FEZRKKUC6Y4GZDIYPSC4SMSALLR665POBPABFM
  ```

10. Create a child account
  ```
  Public Key	GCKQ4EJVBJ5YMPVEEYFL4BWHRJZGFYTKGXVQJBTZCZKH4CM3HUQ7V6YJ
  Secret Key	SAE6QSAEAT5PMA6VTMFE6RCYMIKVRMJKNESYK5NJ5W6DZ4ESLJUCEPIH
  ```

11. Establish a trusline between both accounts and the created token (DND)
  - Stellar laboratory

12. Fund parent account with the created token (DND)
  - Stellar laboratory

13. Increase parent account allowance
  ```
  soroban invoke \
    --id 8216da0fc28346cdd27a61819f93cc7c1488584717ba6e94a8e45a5daa084688 \
    --secret-key SDVSMGYKIFR3MLUPY5FEZRKKUC6Y4GZDIYPSC4SMSALLR665POBPABFM \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn incr_allow \
    --arg '{"object":{"vec":[{"symbol":"Invoker"}]}}' \
    --arg 0 \
    --arg '{"object":{"vec":[{"symbol":"Contract"},{"object":{"bytes":"07cc3fbe8350d9f4135b6065f3dc9599a12d58b1266223ff7cc259c429008a54"}}]}}' \
    --arg 300000
  ```

14. Invoke Parent Allowance contract initialization
  ```
  soroban invoke \
    --id 07cc3fbe8350d9f4135b6065f3dc9599a12d58b1266223ff7cc259c429008a54 \
    --secret-key SDVSMGYKIFR3MLUPY5FEZRKKUC6Y4GZDIYPSC4SMSALLR665POBPABFM \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn initialize \
    --arg '{"object":{"account_id":{"public_key_type_ed25519":"6c941fb51890a054ae4ab8dec4fd77db9d7e3baf9d24993c4d20eedc3ea7f62b"}}}' \
    --arg 8216da0fc28346cdd27a61819f93cc7c1488584717ba6e94a8e45a5daa084688 \
    --arg 300 \
    --arg 0 \
    --arg 0
  ```

15. Test getStep
  ```
  soroban invoke \
    --id 07cc3fbe8350d9f4135b6065f3dc9599a12d58b1266223ff7cc259c429008a54 \
    --secret-key SDVSMGYKIFR3MLUPY5FEZRKKUC6Y4GZDIYPSC4SMSALLR665POBPABFM \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn getStepP
  ```

16. Test setAllow
  ```
  soroban invoke \
    --id 07cc3fbe8350d9f4135b6065f3dc9599a12d58b1266223ff7cc259c429008a54 \
    --secret-key SDVSMGYKIFR3MLUPY5FEZRKKUC6Y4GZDIYPSC4SMSALLR665POBPABFM \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn setAllow \
    --arg '{"object":{"account_id":{"public_key_type_ed25519":"950e11350a7b863ea4260abe06c78a7262e26a35eb04867916547e099b3d21fa"}}}' \
    --arg 1000
  ```

17. Test withdraw
  ```
  soroban invoke \
    --id 07cc3fbe8350d9f4135b6065f3dc9599a12d58b1266223ff7cc259c429008a54 \
    --secret-key SAE6QSAEAT5PMA6VTMFE6RCYMIKVRMJKNESYK5NJ5W6DZ4ESLJUCEPIH \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn withdraw \
    --arg '{"object":{"account_id":{"public_key_type_ed25519":"950e11350a7b863ea4260abe06c78a7262e26a35eb04867916547e099b3d21fa"}}}' \
    --arg 700
  ```

17. Test getWthdrwn
  ```
  soroban invoke \
    --id 07cc3fbe8350d9f4135b6065f3dc9599a12d58b1266223ff7cc259c429008a54 \
    --secret-key SAE6QSAEAT5PMA6VTMFE6RCYMIKVRMJKNESYK5NJ5W6DZ4ESLJUCEPIH \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn getWthdrwn \
    --arg '{"object":{"account_id":{"public_key_type_ed25519":"950e11350a7b863ea4260abe06c78a7262e26a35eb04867916547e099b3d21fa"}}}'
  ```