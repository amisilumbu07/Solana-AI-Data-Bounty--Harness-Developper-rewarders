use anchor_lang::prelude::*;

declare_id!("7xMSusZbcKcgQy8UGbUnAzjy64hPrFSvbUFUzWjXKVDV");

#[program]
pub mod ai_data_bounty {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
