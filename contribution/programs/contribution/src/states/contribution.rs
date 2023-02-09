use crate::errors::ContributionError;
use _profile::states::TechnologyCategroy;
use anchor_lang::prelude::*;

#[account]
pub struct PlusToken {
    pub amount: u32,
}

#[account]
pub struct Contribution {
    pub title: String,
    pub xp_gains: XpGains,
    pub point_token_gains: u32,
    pub paticipants: Vec<Pubkey>,
    pub proposer: Pubkey,
    pub is_completed: bool,
    pub is_active: bool,
    pub category: ContributionCategory,
    pub reward_token: RewardToken,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub enum ContributionCategory {
    Quest,
    Governance,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct XpGains {
    pub program: Pubkey,
    pub skill_gains: Vec<SkillGains>,
    pub claimed: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub struct SkillGains {
    pub skill: TechnologyCategroy,
    pub xp: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub struct RewardToken {
    pub mint_account: Pubkey,
    pub amount: u64,
}

impl Contribution {
    pub const MAX_SIZE: usize = 32 // max title length
    + ( (1 + 5) * 4 + 32)// xp gains (max skills to gain for : 4)
    + 4 // plus token gains
    + (4 + 10 * 32) // max participants : 10
    + 32 // proposer
    + 1 // completed or not
    + 1 // active or not
    + 1; // contribution category

    pub fn initialize(
        &mut self,
        title: String,
        skill_gains: Vec<SkillGains>,
        point_token_gains: u32,
        proposer: Pubkey,
        xp_program: Pubkey,
        category: ContributionCategory,
        reward_token: RewardToken,
    ) {
        self.title = title;
        self.point_token_gains = point_token_gains;
        self.paticipants = Vec::new();
        self.proposer = proposer;
        self.is_completed = false;
        self.is_active = true;
        self.xp_gains = XpGains {
            program: xp_program,
            skill_gains: skill_gains,
            claimed: false,
        };
        self.category = category;
        self.reward_token = reward_token;
    }
}

impl Contribution {
    pub fn set_participant(&mut self, participant: Pubkey) -> Result<()> {
        require!(
            self.is_completed == false,
            ContributionError::CompletedContribution
        );
        require!(
            self.is_active == true,
            ContributionError::InactiveContribution
        );
        require!(
            self.paticipants.len() < 10,
            ContributionError::ExceededParticipants
        );
        require!(
            self.paticipants.iter().find(|&&p| p == participant) == None,
            ContributionError::DuplicatedParticipant
        );

        self.paticipants.push(participant);

        Ok(())
    }

    pub fn complete(&mut self, proposer: Pubkey) -> Result<()> {
        require!(
            self.proposer == proposer,
            ContributionError::InvalidProposer
        );
        require!(
            self.is_completed == false,
            ContributionError::CompletedContribution
        );
        require!(
            self.is_active == true,
            ContributionError::InactiveContribution
        );

        self.is_completed = true;
        self.is_active = false;

        Ok(())
    }

    pub fn deactivate(&mut self, proposer: Pubkey) -> Result<()> {
        require!(
            self.proposer == proposer,
            ContributionError::InvalidProposer
        );
        require!(
            self.is_completed == false,
            ContributionError::CompletedContribution
        );
        require!(
            self.is_active == true,
            ContributionError::CompletedContribution
        );

        self.is_active = false;

        Ok(())
    }

    pub fn activate(&mut self, proposer: Pubkey) -> Result<()> {
        require!(
            self.proposer == proposer,
            ContributionError::InvalidProposer
        );
        require!(
            self.is_completed == false,
            ContributionError::CompletedContribution
        );
        require!(
            self.is_active == false,
            ContributionError::CompletedContribution
        );

        self.is_active = true;

        Ok(())
    }
}

impl Contribution {
    fn check_completed(&self) -> bool {
        self.is_active == false && self.is_completed == true
    }

    fn check_xp_claimed(&self) -> bool {
        self.xp_gains.claimed
    }

    pub fn set_xp_claimed(&mut self) -> Result<()> {
        require!(
            self.check_completed() == true,
            ContributionError::NotCompletedContribution
        );
        require!(
            self.check_xp_claimed() == false,
            ContributionError::AlreadyClaimed
        );

        self.xp_gains.claimed = true;
        Ok(())
    }
}

// For test
impl Contribution {
    pub fn new(
        title: String,
        skill_gains: Vec<SkillGains>,
        point_token_gains: u32,
        proposer: Pubkey,
        xp_program: Pubkey,
        category: ContributionCategory,
        reward_token: RewardToken,
    ) -> Self {
        Self {
            title,
            xp_gains: XpGains {
                program: xp_program,
                skill_gains,
                claimed: false,
            },
            point_token_gains,
            paticipants: Vec::new(),
            proposer,
            is_completed: false,
            is_active: true,
            category,
            reward_token,
        }
    }
}
