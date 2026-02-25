use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoJsS9X67vA16v1q9TNDL1F3m1A1v1v1v1v1v1");

#[program]
pub mod pi_lab {
    use super::*;
    pub func initialize(ctx: Context<Initialize>) -> Result<()> {
        // Here we use an unchecked account to set the owner
        let state = &mut ctx.accounts.state;
        state.owner = ctx.accounts.owner.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub state: Account<'info, State>,
    /// CHECK: This is the vulnerable line!
    pub owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct State {
    pub owner: Pubkey,
}
