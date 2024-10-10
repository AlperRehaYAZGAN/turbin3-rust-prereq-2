pub mod programs;

#[cfg(test)]
mod tests {
    use bs58;
    use solana_sdk::message::Message;
    use solana_sdk::system_program; // to install: cargo add bs58
    use std::io::{self, BufRead};
    use std::str::FromStr;

    use crate::programs::turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram}; // UpdateArgs
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_instruction::transfer,
        transaction::Transaction,
    };
    use std::env;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdop() {
        // Import our keypair
        let keypair = read_keypair_file("keys/dev-wallet.json").expect("Couldn't find wallet file");
        // we'll establish a connection to Solana devnet using the const we defined above

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // Finally, we're going to call the airdrop function:
        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("✅ Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        }
    }

    #[test]
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("keys/dev-wallet.json").expect("Couldn't find wallet file");

        // get from env
        let to_address = env::var("TO_ADDRESS").expect("TO_ADDRESS must be set");
        let to_pubkey = Pubkey::from_str(&to_address).unwrap();

        // Now let's create a connection to devnet
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // In order to sign transactions, we're going to need to get a recent blockhash, as signatures are designed to expire as a security feature:

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // SECTION-1: Send 1 SOL
        // let transaction = Transaction::new_signed_with_payer(
        //     &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
        //     Some(&keypair.pubkey()),
        //     &vec![&keypair],
        //     recent_blockhash,
        // );

        // // Send the transaction
        // let signature = rpc_client
        // .send_and_confirm_transaction(&transaction)
        //     .expect("Failed to send transaction");

        // println!(
        //     "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        //     signature
        // );

        // SECTION-2: Send Remaining SOL
        // Get balance of dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "✅ Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn enroll() {
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // get from env
        let github_slug = env::var("GITHUB_SLUG").expect("GITHUB_SLUG must be set");
        let slug = github_slug.as_bytes();
        let turbin3_bump_seed = env::var("BUMP_SEED").expect("BUMP_SEED must be set");
        let bump_seed = turbin3_bump_seed.as_bytes();
        let turbine_wallet_b58 =
            env::var("TURBIN3_WALLET_SECRET_B58").expect("TURBIN3_WALLET_SECRET_B58 must be set");

        // Decode our wallet
        let turbine_wallet_vec = bs58::decode(turbine_wallet_b58).into_vec().unwrap();

        // Let's define our accounts
        let signer = Keypair::from_bytes(&turbine_wallet_vec).unwrap();

        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            bump_seed,
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define our instruction data
        let args = CompleteArgs {
            github: slug.to_vec(),
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Now we can invoke the "complete" function
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print our transaction out
        println!(
            "✅ Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
