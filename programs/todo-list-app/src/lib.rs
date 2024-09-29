use anchor_lang::prelude::*;

declare_id!("6bG1wLW5oSvAtzWcZmW4HAfcXRgBJerghLoXHYSeFg1T");

#[program]
pub mod todo_list_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
