use anchor_lang::prelude::*;

declare_id!("31bSr5Arjmv8mJd9Ub2KgQMwzGpRXuZr35CnetCBPJxM");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You sent {} and {}", a, b);
        // let x: u64 = a - b; // will silently overflow
        let x = a.checked_sub(b).unwrap();
        msg!("the sub a-b result is {}", x);  
        msg!("{:?}", message);
        Ok(())
    }

    // added this function
    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
