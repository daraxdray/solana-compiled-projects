// use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::{
// account_info::{next_account_info, AccountInfo},
// entrypoint::{self, ProgramResult},
// msg,
// program_error::ProgramError,
//  pubkey::Pubkey
// };
// use solana_program_test::*;
// use solana_sdk::{
//     account::Account,
//     instruction::{AccountMeta, Instruction},
//     pubkey::Pubkey,
//     signature::Signer,
//     transaction::Transaction,
// };

// #[derive(BorshSerialize, BorshDeserialize, Debug)]
// pub struct GreetingAccount {
//     pub counter: u32,
// }

// fn main() {
//     println!("Hello, world!");
// }

// // entrypoint!(process_instruction);
// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data:&[u8],
// )-> ProgramResult{
//     let accounts_iter = &mut accounts.iter();

//     //Get the account to say hello to
//     let account = next_account_info(accounts_iter)?;

//     if account.owner != program_id {
//         msg!("Greeted account does not have the correct program id");
//         return Err(ProgramError::IncorrectProgramId);
//     }
//     //increment and store the number of times the account has been greeted
//     let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
//     greeting_account.counter += 1;
//     greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

//     // let greeting_array = account.data.try_borrow_mut().unwrap();
//     // let mut greeting_data: GreetingAccount = GreetingAccount::try_from_slice(&greeting_array[..]).unwrap();
//     // greeting_data.counter+=1;

//     // greeting_array[..].copy_from_slice(&greeting_data.try_to_vec().unwrap());

//     msg!("Greeted {} time(s)!", greeting_account.counter);
//     Ok(())
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;
//     use std::mem;

//     #[test]
//     fn test_sanity() {
//         let program_id = Pubkey::default();
//         let key = Pubkey::default();
//         let mut lamports = 0;
//         let mut data = vec![0; mem::size_of::<u32>()];
//         let owner = Pubkey::default();
//         let account = AccountInfo::new(
//             &key,
//             false,
//             true,
//             &mut lamports,
//             &mut data,
//             &owner,
//             false,
//             Epoch::default(),
//         );
//         let instruction_data: Vec<u8> = Vec::new();

//         let accounts = vec![account];

//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             0
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             1
//         );
//         process_instruction(&program_id, &accounts, &instruction_data).unwrap();
//         assert_eq!(
//             GreetingAccount::try_from_slice(&accounts[0].data.borrow())
//                 .unwrap()
//                 .counter,
//             2
//         );
//     }
// }



// use solana_client::rpc_client::RpcClient;
// use solana_program::system_instruction;
// use solana_sdk::commitment_config::CommitmentConfig;
// use solana_sdk::native_token::LAMPORTS_PER_SOL;
// use solana_sdk::signature::{Keypair, Signer};

// fn main() {
//     //Create an HTTP RpcClient with specified "confirmed" commitment level confirmed; - the node will query the most recent block that has been voted on by super majroity of the cluster.
//     let rpc_url = String::from("https://api.devnet.solana.com");
//     let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());


//     //Generate fee payer and new account key pairs
//     let fee_payer = Keypair::new();
//     let new_account = Keypair::new();

//     //Request an airdrop foro the fee payer and wait for the transaction to be confirmed
//     let request_airdrop_tx_signature = rpc_client
//                                         .request_airdrop(&fee_payer.pubkey(),LAMPORTS_PER_SOL)
//                                         .unwrap();

//     loop {
//         if let ok(confirmed) = rpc_client.confirm_transaction(&request_airdrop_tx_signature) {
//             if confirmed {
//                 break;
//             }
//         }
//     }

//     //specify account data length
//     let space: u64 = 0;

//     //Get minimum balance required to make an account with specified data length rent exempt
//     let rent_exemption_amount = rpc_client
//                                 .get_minimum_balance_for_rent_exemption(space)
//                                 .unwrap();
//     let create_account_ix = system_instruction::create_account(
//         &fee_payer.pubkey(),
//         &new_account.pubkey(),
//         rent_exemption_amount,
//         space,
//         &fee_payer.pubkey()
//     );                                              
//     //Get recent blockhash
//     let recent_blockhash = rpc_client
//                             .get_latest_blockhash()
//                             .unwrap();

//     //Create transaction to create an account
//     let create_account_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
//         &[create_account_ix],
//         Some(&fee_payer.pubkey()),
//         &[&fee_payer, &new_account],
//         recent_blockhash
//     );

//     //Submit a transaction to create an account and wait for confirmation
//     let create_account_tx_signature = rpc_client
//                                       .send_and_confirm_transaction(&create_account_tx)
//                                       .unwrap();  
    
//     // Print transaction signature and account address
//     println!("Transaction signature: {create_account_tx_signature}");
//     println!("New account {} created successfully", new_account.pubkey());                                 

// }

