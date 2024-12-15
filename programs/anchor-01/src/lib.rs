use anchor_lang::prelude::*;

declare_id!("5V5WHfnSe6DFPvGDttgRkPYqjjYoBxVUeRjgsi3Ja2Ly");

#[program]
pub mod anchor_01 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
