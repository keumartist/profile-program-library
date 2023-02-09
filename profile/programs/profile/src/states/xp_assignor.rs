use anchor_lang::prelude::*;

use super::TechnologyCategory;
use crate::errors::XpAssignorError;

#[account]
pub struct XpAssignor {
    pub assignor_keys: Vec<Pubkey>,
    pub contribution_key: Pubkey,
    pub xp_assigns: Vec<XpAssign>,
    pub assignable: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct XpAssign {
    pub xp_gains: Vec<XpGains>,
    pub assignee: Pubkey,
    pub claimed: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct XpGains {
    pub skill: TechnologyCategory,
    pub xp: u32,
}

impl XpAssignor {
    pub const MAX_SIZE: usize = (4 + (32 * 4)) // max assignors : 4
        + 32 // contribution pubkey
        + (4 + 10 * (1 + 4 + 32 + (4 * (1 + 4)))); // vec of xp assign. max assignees : 10. max xp gains: 4
}

impl XpAssignor {
    pub fn new(
        assignor_keys: Vec<Pubkey>,
        contribution_key: Pubkey,
        xp_assigns: Vec<XpAssign>,
    ) -> Self {
        Self {
            assignor_keys,
            contribution_key,
            xp_assigns,
            assignable: false,
        }
    }
}

impl XpAssignor {
    pub fn initialize(
        &mut self,
        assignor_keys: Vec<Pubkey>,
        contribution_key: Pubkey,
        xp_assigns: Vec<XpAssign>,
    ) -> Result<()> {
        require!(assignor_keys.len() > 0, XpAssignorError::EmptyAssginors);
        require!(xp_assigns.len() > 0, XpAssignorError::EmptyXpAssigns);

        self.assignor_keys = assignor_keys;
        self.contribution_key = contribution_key;
        self.xp_assigns = xp_assigns;
        self.assignable = false;

        Ok(())
    }

    pub fn update(
        &mut self,
        assignor_keys: Vec<Pubkey>,
        contribution_key: Pubkey,
        xp_assigns: Vec<XpAssign>,
    ) -> Result<()> {
        require!(assignor_keys.len() > 0, XpAssignorError::EmptyAssginors);
        require!(xp_assigns.len() > 0, XpAssignorError::EmptyXpAssigns);

        self.assignor_keys = assignor_keys;
        self.contribution_key = contribution_key;
        self.xp_assigns = xp_assigns;
        self.assignable = false;

        Ok(())
    }

    pub fn set_assignable(&mut self, assignor_key: &Pubkey) -> Result<()> {
        require!(self.assignable == false, XpAssignorError::AlreadyAssignable);
        require!(
            self.assignor_keys
                .iter()
                .find(|&assignor| assignor == assignor_key)
                != None,
            XpAssignorError::InvalidAssignor
        );

        self.assignable = true;

        Ok(())
    }
}

impl XpAssignor {
    pub fn set_claimed(&mut self, assignee_key: Pubkey) -> Result<()> {
        self.validate_claim(assignee_key).unwrap();

        let xp_assign = self
            .xp_assigns
            .iter_mut()
            .find(|xp| xp.assignee == assignee_key)
            .unwrap();

        xp_assign.claimed = true;

        Ok(())
    }

    fn validate_claim(&mut self, assignee_key: Pubkey) -> Result<()> {
        require!(self.assignable == true, XpAssignorError::NotAssignable);

        let xp_assign_option = self
            .xp_assigns
            .iter()
            .find(|&xp| xp.assignee == assignee_key);

        require!(xp_assign_option != None, XpAssignorError::InvalidAssignee);
        require!(
            xp_assign_option.unwrap().claimed == false,
            XpAssignorError::AlreadyClaimed
        );

        Ok(())
    }
}
