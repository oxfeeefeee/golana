use {
    golana::processor::process_instruction,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar::{self},
    },
    solana_program_test::*,
    solana_sdk::{signature::Signer, transaction::Transaction},
    std::str::FromStr,
};

#[tokio::test]
async fn test_sysvar() {
    let program_id = Pubkey::from_str("Go1ana1111111111111111111111111111111111111").unwrap();
    let (mut banks_client, payer, recent_blockhash) =
        ProgramTest::new("golana_test1", program_id, processor!(process_instruction))
            .start()
            .await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &(),
            vec![
                AccountMeta::new(sysvar::clock::id(), false),
                AccountMeta::new(sysvar::rent::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}
