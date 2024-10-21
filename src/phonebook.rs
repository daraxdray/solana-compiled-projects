use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg,
    sysvar::rent::Rent,
    program_pack::{Pack,Sealed},
    borsh::{BorshDeserialize, BorshSerialize}
};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressBook {
    pub entries: Vec<Pubkey,String>,

}

impl Sealed for AddressBook{}

impl Pack for AddressBook{
    const LEN: usize = 23406 * (32 + 14);

    fn unpack_from_slice(input: &[u8]) -> Result <Self, ProgramError> {
        AddressBook::try_from_slice(input).map_err(|_| ProgramError::InvalidAccountData)
    }


    fn pack_into_slice(&self, output: &mut [u8]){
        self.serialize(output).unwrap();
    }
}


// entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account = &accounts[0];
    let mut data = AddressBook::try_from_slice(&account.data.borrow())?;

        // Check if the instruction data length matches the expected length for an add operation (32-byte Pubkey + 14 chars for phone number)
    if instruction_data.len() != (32 + 14){

        msg!("Invalid Length of instruction data");
        return Err(ProgramError::InvalidInstructionData);
    }

        // Extract the pubkey and phone number from instruction data
    

    //check account
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgranId)
    }

    //deserialize the account data
    let acc_data = AddressBook::try_from_slice(account.data.borrow())?;


    //Logic to add or update mappings in th address book
    initialize_address_book();

    AddressBook::pack(data, &mut account.data.borrow_mut())?;
    Ok(())
}


fn generate_address_book() -> AddressBook {
    let mut keys = Vec::with_capacity(23406);
    let mut phone_numbers = Vec::with_capacity(23406);

    for _ in 0..23406 {
        let pubkey = Pubkey::new_unique();
        let phone_numbers = generate_random_phone_number();
        keys.push(pushkey);
        phone_numbers.push(push_number);
    }

    AddressBook { keys, phone_numbers }

}


fn generate_random_phone_number()-> String {
    let mut rng = rand::thread_rng();
    let digits: String = (0..14).map(
        |_| rng.gen_range(0..10).to_string()
    ).collect();
    digits
}

fn save_address_book(address_book: &AddressBook,
account: &AccountInfo,) -> ProgramResult {
     let mut data = account.try_borrow_mut_data()?;
     AddressBook::pack(address_book, &mut data)?;
     Ok(())
}

pub fn initialize_address_book(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account = &accounts[0];

    let rent = Rent::get()?;
    if !rent.is_exempt(account.lamports(), account.data_len()){
        return Err(ProgramError::AccountNotRentExempt)
    }

    // Generate the AddressBook with 23,406 unique keys and phone numbers
    let address_book = generate_address_book();

    //save
    save_address_book(&address_book, account)?;

    msg!("Address book initialized and saved!");
    Ok(())

}


