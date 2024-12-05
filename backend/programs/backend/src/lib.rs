use anchor_lang::prelude::*;

declare_id!("Aj9j6H3EQ4qtaJJ3BwU3EVVrDGGQA4WRQswsBiamtVWH");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
