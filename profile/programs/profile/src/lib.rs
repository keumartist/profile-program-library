pub mod errors;
pub mod instructions;
pub mod states;

use anchor_lang::prelude::*;
use instructions::*;
use states::*;

declare_id!("7heeP34yJJ8aWhzG72RJAiYaJX48djqhbfhBn9ccoiGf");

#[program]
pub mod _profile {
    use super::*;

    pub fn create_everlastings(ctx: Context<CreateEverlastings>) -> Result<()> {
        instructions::everlastings::create_everlastings(ctx)
    }

    pub fn claim_social_badge_persnality_mbti(
        ctx: Context<ClaimSocialBadge>,
        mbti_index: u8,
    ) -> Result<()> {
        instructions::social_badge::claim_social_badge_persnality_mbti(ctx, mbti_index)
    }

    pub fn claim_social_badge_soft_skill(
        ctx: Context<ClaimSocialBadge>,
        soft_skill_index: u8,
    ) -> Result<()> {
        instructions::social_badge::claim_social_badge_soft_skill(ctx, soft_skill_index)
    }

    pub fn claim_skill_badge_position(
        ctx: Context<ClaimSkillBadge>,
        skill_position_index: u8,
    ) -> Result<()> {
        instructions::claim_skill_badge_position(ctx, skill_position_index)
    }

    pub fn claim_skill_badge_technology(
        ctx: Context<ClaimSkillBadge>,
        technology_index: u8,
    ) -> Result<()> {
        instructions::claim_skill_badge_technology(ctx, technology_index)
    }

    pub fn create_xp_assignor(
        ctx: Context<CreateXpAssignor>,
        contribution_key: Pubkey,
        xp_assigns: Vec<XpAssign>,
    ) -> Result<()> {
        instructions::create_xp_assignor(ctx, contribution_key, xp_assigns)
    }

    pub fn activate_xp_assignor(ctx: Context<ActivateXpAssignor>) -> Result<()> {
        instructions::activate_xp_assignor(ctx)
    }
}
