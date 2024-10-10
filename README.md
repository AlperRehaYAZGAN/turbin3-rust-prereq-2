### Turbin3 Assessment - 2 Rust IDL Check

#### Environment variable before starting

New Program ID: `HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1`

```bash
# set airdrop target address (your turbin3 public key)
export TO_ADDRESS=<your public-key>
export TURBIN3_WALLET_SECRET_B58=<your private-key-from-phantom-base58>

# add your github username to verify your identity
export GITHUB_SLUG=username

# BUMP_SEED for they pointing pda account
export BUMP_SEED=prereq
```

#### Steps

1. Copy paste your `dev-wallet.json` and `dev-wallet-pubkey.txt` into keys folder.

2. In VSCode run test your airdrop function.

```bash
cargo test --lib -- tests::airdop --exact --show-output
```

```txt
✅ Success! Check out your TX here:
https://explorer.solana.com/tx/5NTPVirkGWjUyPiWxYkrn6nXxTRKdLMXSm3oRmTXiHLqkem718ndf54vWQrVvh1vbQnrsKKC4N1BccSyuRwPBv4y?cluster=devnet

successes:
    tests::airdrop
```

3. Transfer all airdrop from temp wallet into turbin3 wallet

```bash
cargo test --lib -- tests::transfer_sol --exact --show-output
```

```txt
✅ Success! Check out your TX here: https://explorer.solana.com/tx/4RXrup2S4SYRX93q6guERZ2JeVxhtHVaySDZHXCs2csmVcVeAhdPdVBTPpkYaQrJ22SctEm5v8SpJH4GTEuQa5AV/?cluster=devnet

successes:
    tests::transfer_sol
```

4. Execute WBA program to graduate from course

```bash
cargo test --lib -- tests::enroll --exact --show-output
```

```txt
---- tests::enroll stdout ----
✅ Success! Check out your TX here: https://explorer.solana.com/tx/4Q99tw5Fe7ZqpdX1zKC664vFuxmLkQFsVr3cuEmnLpt81zvaBF5mvaGrwDFAyGeseRGaGXoJLCnDSknFQEZXwJCR/?cluster=devnet


successes:
    tests::enroll
```

#### Notes:

Do not forget to add metadata.address into idlgen! macro.

```rust
idlgen!({
"version": "0.1.0", "name": "turbin3_prereq",
"metadata": {
    "address": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1"
}});
```
