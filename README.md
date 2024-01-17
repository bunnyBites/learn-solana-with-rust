Let's test Solana with the local environment first with Rust.

If you haven't installed Rust, now will be a good time:
https://www.rust-lang.org/tools/install

- Create a new rust project

- Add the following crates to the Cargo.toml file:
  
  solana-client [https://crates.io/crates/solana-client]

  solana-sdk [https://crates.io/crates/solana-sdk]

The following steps are taken from the official Solana documentation:
1. Solana in the local environment - https://solanacookbook.com/getting-started/installation.html#install-cli
2. Keypair and wallets - https://solanacookbook.com/references/keypairs-and-wallets.html#how-to-generate-a-new-keypair
3. solDev course - https://www.soldev.app/course

I'll be guiding you on how to set up a Linux system (terminal):
1. Install Rust:
   
  ``curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh ``

2. Install Solana CLI (this CLI is used to run our program locally):
   
  ``sh -c "$(curl -sSfL https://release.solana.com/stable/install)"``

  If the above command throws a path that you need to add:

  ``export PATH=<THROWN_PATH>:$PATH ``
  
  replace THROWN_PATH with the path you received in step 2.

3. Check if the local setup is complete:
    `` solana --version ``

  If you get an error, you can restart your terminal or system to ensure the PATH is updated
  You can also check your PATH by running this command in the terminal:
  
  ``echo $PATH ``

  The path should have something like this:
  
  `` /home/bunny/.local/share/solana/install/active_release/bin ``

4. Test your local-test-validator:

  ``solana-test-validator ``

   If your setup succeeds, you will get many configuration details. We are only interested in:
    `` JSON RPC URL: http://127.0.0.1:8899 ``

  This URL is what we need to pass in the code.

5. You have successfully configured Solana in your local environment. You can now check the ``main.rs`` to get started.
