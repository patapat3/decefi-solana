# WIP
WARNING: UNFINISHED, WIP
--------

# decefi-solana

https://decefi.app

## Deploying decefi
```
# compile the decefi binary
./do.sh build decefi

# deploy the decefi to the configured solana cluster
DECEFI_PROGRAM_ID="$(solana deploy decefi/target/bpfel-unknown-unknown/release/decefi.so | jq .programId -r)"
```
