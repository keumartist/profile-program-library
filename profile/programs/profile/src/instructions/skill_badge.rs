use anchor_lang::prelude::*;

use crate::states::{
    Everlastings, SkillPositionCategory, Technology, TechnologyCategory, XpAssignor,
};

pub fn claim_skill_badge_position(
    ctx: Context<ClaimSkillBadge>,
    skill_position_index: u8,
) -> Result<()> {
    let everlastings_account = &mut ctx.accounts.everlastings_account;
    let skill_position_category =
        SkillPositionCategory::try_from_slice(&[skill_position_index] as &[u8])?;

    msg!("Claimed skill position is {:?}", &skill_position_category);
    everlastings_account.set_skill_badge_position(skill_position_category);

    Ok(())
}

pub fn claim_skill_badge_technology(
    ctx: Context<ClaimSkillBadge>,
    technology_index: u8,
) -> Result<()> {
    let everlastings_account = &mut ctx.accounts.everlastings_account;
    let technology_category = TechnologyCategory::try_from_slice(&[technology_index] as &[u8])?;

    msg!("Claimed technology is {:?}", &technology_category);
    everlastings_account.set_skill_badge_technology(technology_category);

    Ok(())
}

pub fn assign_xp_to_skill_badge(
    ctx: Context<AssignXp>,
    skill_technologies: Vec<Technology>,
) -> Result<()> {
    let everlasting_account = &mut ctx.accounts.everlastings_account;
    let xp_assignor_account = &mut ctx.accounts.xp_assignor_account;
    let assignee_key = ctx.accounts.signer.key();

    xp_assignor_account.set_claimed(assignee_key).unwrap();

    msg!("Xps for skill in everlastings assigned");
    everlasting_account
        .assign_skill_xp(skill_technologies)
        .unwrap();

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimSkillBadge<'info> {
    #[account(mut)]
    pub everlastings_account: Account<'info, Everlastings>,
    pub wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AssignXp<'info> {
    #[account(mut)]
    pub everlastings_account: Account<'info, Everlastings>,

    #[account(mut)]
    pub xp_assignor_account: Account<'info, XpAssignor>,
    pub signer: Signer<'info>,
}
