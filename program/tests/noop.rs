use {
    mollusk_svm::{result::Check, Mollusk},
    solana_account::Account,
    solana_instruction::AccountMeta,
    solana_pubkey::Pubkey,
    spl_instruction_padding::instruction::noop,
};

#[test]
fn success_with_noop() {
    let program_id = Pubkey::new_unique();
    let mollusk = Mollusk::new(&program_id, "spl_instruction_padding");

    let first_address = Pubkey::new_unique();
    let second_address = Pubkey::new_unique();
    let third_address = Pubkey::new_unique();

    let padding_accounts = vec![
        AccountMeta::new_readonly(first_address, false),
        AccountMeta::new_readonly(second_address, false),
        AccountMeta::new_readonly(third_address, false),
    ];

    let padding_data = 800;

    mollusk.process_and_validate_instruction(
        &noop(program_id, padding_accounts, padding_data).unwrap(),
        &[
            (first_address, Account::default()),
            (second_address, Account::default()),
            (third_address, Account::default()),
        ],
        &[Check::success()],
    );
}
