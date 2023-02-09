use anchor_lang::prelude::*;
use instructions::*;
use states::*;

pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("EDgdY2aFC4DLBvZcLkvn9qGhPqbo7BizVxJNAc79Q4Rd");

#[program]
pub mod contribution {
    use crate::states::SkillGains;

    use super::*;

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
        instructions::create_contribution(
            ctx,
            title,
            skill_gains,
            point_token_gains,
            proposer,
            xp_program,
            category_index,
            reward_token_amount,
        )
    }

    pub fn participate_contribution(ctx: Context<ParticipateContribution>) -> Result<()> {
        instructions::participate_contribution(ctx)
    }

    pub fn complete_contribution(ctx: Context<CompleteContribution>) -> Result<()> {
        instructions::complete_contribution(ctx)
    }

    pub fn activate_contribution(ctx: Context<ActivateContribution>) -> Result<()> {
        instructions::activate_contribution(ctx)
    }

    pub fn deactivate(ctx: Context<DeactivateContribution>) -> Result<()> {
        instructions::deactivate_contribution(ctx)
    }
}
