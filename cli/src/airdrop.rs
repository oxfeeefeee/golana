use crate::config::Provider;
use crate::util::new_vm_program;
use anyhow::Result;

/// airdrop SOL to wallet
pub fn airdrop(amount: u64, provider: &Provider) -> Result<()> {
    let program = new_vm_program(provider)?;
    let rpc_client = program.rpc();

    let balance = rpc_client.get_balance(&program.payer())?;
    println!("Balance before airdrop: {}", balance);

    let sig = rpc_client.request_airdrop(&program.payer(), amount)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }

    let balance = rpc_client.get_balance(&program.payer())?;
    println!("Balance after airdrop: {}", balance);
    Ok(())
}
