# Let's test Solana with the local environment first with Rust.

If you haven't installed [Rust](https://www.rust-lang.org/tools/install), now will be a good time:

- Create a new rust project

- Add the following crates to the Cargo.TOML file:

  1. [solana-client](https://crates.io/crates/solana-client)

  2. [solana-sdk](https://crates.io/crates/solana-sdk)

The following steps are taken from the official Solana documentation:
1. [Solana in the local environment](https://solanacookbook.com/getting-started/installation.html#install-cli)
2. [Keypair and wallets](https://solanacookbook.com/references/keypairs-and-wallets.html#how-to-generate-a-new-keypair)
3. [SolDev course](https://www.soldev.app/course)

## Setup for Linux system (terminal):
1. Install Rust:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

2. Install Solana CLI (this CLI is used to run our program locally):

  ```bash
  sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
  ```

  If the above command throws a path that you need to add:

  ```bash
  export PATH=<THROWN_PATH>:$PATH
  ```

  replace THROWN_PATH with the path you received in step 2.

3. Check if the local setup is complete:
  ```bash
    solana --version
  ```

  If you get an error, you can restart your terminal or system to ensure the PATH is updated
  You can also check your PATH by running this command in the terminal:

  ```bash
  echo $PATH
  ```

  The path should have something like this:

  `` /home/bunny/.local/share/solana/install/active_release/bin ``

4. Test your local-test-validator:

  ```bash
    solana-test-validator
  ```

   If your setup succeeds, you will get many configuration details:

  ```bash
    --faucet-sol argument ignored, ledger already exists
    Ledger location: test-ledger
    Log: test-ledger/validator.log⠂ Initializing...                                                                               Waiting for fees to stabilize 1...
    Identity: 4QtjGtBQ5kfjfRq6yUqqaWngQ1oQoUGyozBZbbTKhyfi
    Genesis Hash: 5w6HtVfS1a9ESXLAG6TywZhCEFn9nVDe8oiB7cK7j8ij
    Version: 1.16.28
    Shred Version: 12150
    Gossip Address: 127.0.0.1:1024
    TPU Address: 127.0.0.1:1027
    JSON RPC URL: http://127.0.0.1:8899 -----> * URL passed to our code *
    WebSocket PubSub URL: ws://127.0.0.1:8900
    ⠒ 00:00:12 | Processed Slot: 1830 | Confirmed Slot: 1830 | Finalized Slot: 1798 | Full Snapshot Slot: 1702 | Incremental Snapshot Slot: - | Tr
  ```

  This URL is what we need to pass in the code.

5. You have successfully configured Solana in your local environment. You can now check the ``main.rs`` to get started.

6. Output for our program:
```bash
SOLs Before airdropping -> 0
Transaction: 4WxHEwzZL6xv12b4rYALi4oV89TPNL93KXfZ1YSDwJwdHSweu Status: true
SOLs After airdropping -> 1
```

## Creating keypair
Creating keypair is required throughout solana transactions.
It's better to store specific values like public key and secret key in environment variables.


### create a new keypair (wallet):
```bash
let keypair = Keypair::new();
```

### get public keypair
```bash
let pubkey = Signer::pubkey(&keypair)
```

### get secret for our keypair
```bash
let secret = keypair.to_base58_string()
```

### Sample .env file
```sh
MY_PUB_KEY=JBFs2brb5KrFH1D9HqX2YF5JigySowHPN79gBSaoxKrR

OTHER_PUB_KEY=4uyGibipBrgE1Svsai4fg1BTaBWBMRTtxJKX7agFAdLe

MY_SECRET_KEY=28rUCoiLSWGdH6ToJU7tzrjS6oU5u9MbeRJhBh7HN4h26E1w8uZCigJVArNssFEnH3gzak2DSiYQfBWaHyN5FjGu
```