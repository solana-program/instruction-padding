use {
    mollusk_svm::{result::Check, Mollusk},
    solana_account::Account,
    solana_instruction::AccountMeta,
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction as system_instruction,
    spl_instruction_padding::instruction::wrap_instruction,
};

#[test]
fn success_with_padded_transfer_data() {
    let program_id = Pubkey::new_unique();
    let mollusk = Mollusk::new(&program_id, "spl_instruction_padding");

    let from = Pubkey::new_unique();
    let from_account = Account {
        lamports: LAMPORTS_PER_SOL * 2,
        ..Default::default()
    };
    let to = Pubkey::new_unique();

    let transfer_amount = LAMPORTS_PER_SOL;
    let transfer_instruction = system_instruction::transfer(&from, &to, transfer_amount);

    let first_padding_address = Pubkey::new_unique();
    let second_padding_address = Pubkey::new_unique();
    let third_padding_address = Pubkey::new_unique();
    let padding_accounts = vec![
        AccountMeta::new_readonly(first_padding_address, false),
        AccountMeta::new_readonly(second_padding_address, false),
        AccountMeta::new_readonly(third_padding_address, false),
    ];

    let padding_data = 800;

    mollusk.process_and_validate_instruction(
        &wrap_instruction(
            program_id,
            transfer_instruction,
            padding_accounts,
            padding_data,
        )
        .unwrap(),
        &[
            (from, from_account),
            (to, Account::default()),
            mollusk_svm::program::keyed_account_for_system_program(),
            (first_padding_address, Account::default()),
            (second_padding_address, Account::default()),
            (third_padding_address, Account::default()),
        ],
        &[
            Check::success(),
            Check::account(&to).lamports(transfer_amount).build(),
        ],
    );
}
