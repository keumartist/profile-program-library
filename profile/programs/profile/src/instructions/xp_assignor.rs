use anchor_lang::prelude::*;

use crate::states::{XpAssign, XpAssignor};

pub fn create_xp_assignor(
    ctx: Context<CreateXpAssignor>,
    contribution_key: Pubkey,
    xp_assigns: Vec<XpAssign>,
) -> Result<()> {
    let xp_assignor_account = &mut ctx.accounts.xp_assignor_account;
    let signer = &ctx.accounts.signer;

    xp_assignor_account.initialize(Vec::from([signer.key()]), contribution_key, xp_assigns)
}

pub fn activate_xp_assignor(ctx: Context<ActivateXpAssignor>) -> Result<()> {
    let xp_assignor_account = &mut ctx.accounts.xp_assignor_account;
    let assignor = &ctx.accounts.signer.key();

    xp_assignor_account.set_assignable(assignor)
}

pub fn claim_xp(ctx: Context<ClaimXp>) -> Result<()> {
    let xp_assignor_account = &mut ctx.accounts.xp_assignor_account;
    let assignee_key = ctx.accounts.signer.key();

    xp_assignor_account.set_claimed(assignee_key)
}

#[derive(Accounts)]
pub struct CreateXpAssignor<'info> {
    #[account(init,
        payer = signer,
        space = XpAssignor::MAX_SIZE,
        seeds = [
            signer.key().as_ref(),
            b"xp_assignor",
        ],
        bump
    )]
    pub xp_assignor_account: Account<'info, XpAssignor>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActivateXpAssignor<'info> {
    #[account(mut)]
    pub xp_assignor_account: Account<'info, XpAssignor>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimXp<'info> {
    #[account(mut)]
    pub xp_assignor_account: Account<'info, XpAssignor>,

    #[account(mut)]
    pub signer: Signer<'info>,
}
