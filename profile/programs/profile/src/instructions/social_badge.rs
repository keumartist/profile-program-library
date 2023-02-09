use anchor_lang::prelude::*;

use crate::states::{Everlastings, SoftSkillCategory, MBTI};

pub fn claim_social_badge_persnality_mbti(
    ctx: Context<ClaimSocialBadge>,
    mbti_index: u8,
) -> Result<()> {
    let everlastings_account = &mut ctx.accounts.everlastings_account;
    let mbti = MBTI::try_from_slice(&[mbti_index] as &[u8])?;

    msg!("Claimed MBTI is {:?}", &mbti);
    everlastings_account.set_social_badge_persnality_mbti(mbti);

    Ok(())
}

pub fn claim_social_badge_soft_skill(
    ctx: Context<ClaimSocialBadge>,
    soft_skill_index: u8,
) -> Result<()> {
    let everlastings_account = &mut ctx.accounts.everlastings_account;
    let soft_skill_category = SoftSkillCategory::try_from_slice(&[soft_skill_index] as &[u8])?;

    msg!("Claimed Soft skill is {:?}", &soft_skill_category);
    everlastings_account.set_social_badge_soft_skill(soft_skill_category);

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimSocialBadge<'info> {
    #[account(mut)]
    pub everlastings_account: Account<'info, Everlastings>,

    #[account(mut)]
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
}
