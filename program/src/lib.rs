mod entrypoint;
pub mod instruction;
pub mod processor;

pub use spl_instruction_padding_interface::{check_id, id, ID};
pub use {
    solana_account_info, solana_cpi, solana_instruction, solana_program_entrypoint,
    solana_program_error, solana_pubkey,
};
