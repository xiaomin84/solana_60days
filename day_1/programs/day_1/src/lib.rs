use anchor_lang::prelude::*;

declare_id!("HZuZ3m5DEm6k4w8my6KVm4igTP5xBPURuZE9BxEUzerz");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello, world!"); 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
