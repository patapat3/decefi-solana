use decefi::Decefi;

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &[AccountInfo<'a>],
    instruction_data: &[u8],
) -> DexResult {
    let original_data: Vec<Vec<u8>> = accounts
        .iter()
        .map(|account| account.try_borrow_data().unwrap().to_vec())
        .collect();
    let result = Decefi::process(program_id, accounts, &instruction_data);
    if result.is_err() {
        for (account, original) in accounts.iter().zip(original_data) {
            let mut data = account.try_borrow_mut_data().unwrap();
            data.copy_from_slice(&original);
        }
    }
    result
}