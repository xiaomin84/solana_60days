use anchor_lang::prelude::*;

declare_id!("zt4FHsGngMGF5AFFi2auVWxjxUCjLyDMFTUxv6HswPn");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<BoatyMcBoatface>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn function_b(ctx: Context<Empty>, _first_arg: u64) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct BoatyMcBoatface<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Empty {}
