use crate::states::Everlastings;
use anchor_lang::prelude::*;

pub fn create_everlastings(ctx: Context<CreateEverlastings>) -> Result<()> {
    let everlastings_account = &mut ctx.accounts.everlastings_account;

    everlastings_account.initialize();

    msg!(
        "Created everlastings account : {}",
        &everlastings_account.key()
    );
    Ok(())
}

#[derive(Accounts)]
pub struct CreateEverlastings<'info> {
    #[account(init,
        payer = wallet,
        space = 82,
        seeds = [
            wallet.key().as_ref(),
            b"badge",
        ],
        bump
    )]
    pub everlastings_account: Account<'info, Everlastings>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
}
