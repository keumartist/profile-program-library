use anchor_lang::prelude::*;
use _profile::{
     program::Profile, states::XpAssignor,
};
use anchor_spl::{
    associated_token,
    token,
};

use crate::states::{Contribution, ContributionCategory, SkillGains, RewardToken};

pub fn create_contribution(
    ctx: Context<CreateContribution>,
    title: String,
    skill_gains: Vec<SkillGains>,
    point_token_gains: u32,
    proposer: Pubkey,
    xp_program: Pubkey,
    category_index: u8,
    reward_token_amount: u64,
) -> Result<()> {
    let contribution_account = &mut ctx.accounts.contribution_account;
    let contribution_category = ContributionCategory::try_from_slice(&[category_index] as &[u8])?;

    contribution_account.initialize(
        title,
        skill_gains,
        point_token_gains,
        proposer,
        xp_program,
        contribution_category,
        RewardToken {
            mint_account: ctx.accounts.token_mint_account.key(),
            amount: reward_token_amount
        }
    );
    msg!("New contribution created : {}", &contribution_account.key());

    let _ = &ctx.accounts.create_xp_assignor();
    msg!("Xp assignor account for contribution created : {}", &ctx.accounts.xp_assignor_account.key());

    let _ = &ctx.accounts.create_associated_token_account_for_contribution_pda();
    let _ = &ctx.accounts.transfer_reward_token_to_contribution_pda(reward_token_amount);
    
    msg!("Token to be rewarded transferred to contribution pda atoken account");
    
    Ok(())
}

pub fn participate_contribution(ctx: Context<ParticipateContribution>) -> Result<()> {
    let contribution_account = &mut ctx.accounts.contribution_account;

    contribution_account.set_participant(ctx.accounts.participant.key())
}

pub fn complete_contribution(ctx: Context<CompleteContribution>) -> Result<()> {
    let contribution_account= &mut ctx.accounts.contribution_account;

    let _ = contribution_account.complete(ctx.accounts.proposer.key());
    let _ = &ctx.accounts.give_reward_token_to_participants();

    msg!("Contribution completed");

    Ok(())
}

pub fn activate_contribution(ctx: Context<ActivateContribution>) -> Result<()> {
    let contribution_account = &mut ctx.accounts.contribution_account;

    contribution_account.activate(ctx.accounts.proposer.key())
}

pub fn deactivate_contribution(ctx: Context<DeactivateContribution>) -> Result<()> {
    let contribution_account = &mut ctx.accounts.contribution_account;

    contribution_account.deactivate(ctx.accounts.proposer.key())
}

#[derive(Accounts)]
pub struct CreateContribution<'info> {
    #[account(init, payer = signer, space = 8 + Contribution::MAX_SIZE, seeds = [
        signer.key().as_ref(),
        b"contribution",
    ], bump)]
    pub contribution_account: Account<'info, Contribution>,

    #[account(mut, 
        seeds = [
        signer.key().as_ref(),
        b"xp_assignor",
        ],
        bump,
    )]
    pub xp_assignor_account: Account<'info, XpAssignor>,

    /// CHECK: It'll be created in this instruction
    #[account(mut)]
    pub contribution_pda_associated_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub token_mint_account: Account<'info, token::Mint>,
    #[account(mut)]
    pub proposer_associated_token_account: Account<'info, token::TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub xp_assignor_program: Program<'info, Profile>,
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ParticipateContribution<'info> {
    #[account(mut, constraint = participant.key != &contribution_account.proposer )]
    pub contribution_account: Account<'info, Contribution>,

    pub participant: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteContribution<'info> {
    #[account(mut, constraint = proposer.key == &contribution_account.proposer)]
    pub contribution_account: Account<'info, Contribution>,

    #[account(mut)]
    pub contribution_pda_associated_token_account: Account<'info, token::TokenAccount>,
    pub participant_account: SystemAccount<'info>,

    pub proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ActivateContribution<'info> {
    #[account(mut, constraint = proposer.key == &contribution_account.proposer)]
    pub contribution_account: Account<'info, Contribution>,

    pub proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeactivateContribution<'info> {
    #[account(mut, constraint = proposer.key == &contribution_account.proposer)]
    pub contribution_account: Account<'info, Contribution>,

    pub proposer: Signer<'info>,
}
