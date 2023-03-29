use anchor_lang::prelude::*;

declare_id!("A31bjS1pFNcxkVNXP5LwwGNpejETipZJbXC3XGfGu5QY");

#[program]
pub mod metacamp_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, id: u8) -> Result<()> {
        msg!("Create account id: {}", id.to_string());
        msg!("Creator: {}", ctx.accounts.initializer.key());
        msg!("Name: {}", name);



        let onchain_account = &mut ctx.accounts.onchain_account;
        onchain_account.name = name;
        onchain_account.creator = ctx.accounts.initializer.key();
        onchain_account.id = id;
        onchain_account.is_initialized = true;


        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, id: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [id.to_le_bytes().as_ref(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + 1 + 4 + 1 + name.len() + 32 + 1
    )]
    pub onchain_account: Account<'info, OnChainData>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[account]
pub struct OnChainData {
    pub id: u8,                 // 1
    pub name: String,           // 4 + len()// 1
    pub creator: Pubkey,        // 32
    pub is_initialized: bool    // 1
}