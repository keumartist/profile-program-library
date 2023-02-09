use _profile::cpi::{accounts::CreateXpAssignor, create_xp_assignor as cpi_create_xp_assignor};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::{
    associated_token::{self, Create},
    token::{self, spl_token, Transfer},
};

use crate::{
    errors::ContributionError,
    instructions::{CompleteContribution, CreateContribution},
};

impl CreateContribution<'_> {
    pub fn create_xp_assignor(&self) -> Result<()> {
        let accounts = CreateXpAssignor {
            xp_assignor_account: self.xp_assignor_account.to_account_info().clone(),
            signer: self.signer.to_account_info().clone(),
            system_program: self.system_program.to_account_info().clone(),
        };

        // TODO: fix XpGains parameter
        cpi_create_xp_assignor(
            CpiContext::new(self.xp_assignor_program.to_account_info(), accounts),
            self.contribution_account.to_account_info().key(),
            Vec::new(),
        )
    }

    pub fn create_associated_token_account_for_contribution_pda(&self) -> Result<()> {
        let accounts = Create {
            payer: self.signer.to_account_info(),
            associated_token: self
                .contribution_pda_associated_token_account
                .to_account_info(),
            authority: self.signer.to_account_info(),
            mint: self.token_mint_account.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        associated_token::create(CpiContext::new(
            self.associated_token_program.to_account_info(),
            accounts,
        ))
    }

    pub fn transfer_reward_token_to_contribution_pda(&self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.proposer_associated_token_account.to_account_info(),
            to: self
                .contribution_pda_associated_token_account
                .to_account_info(),
            authority: self.proposer_associated_token_account.to_account_info(),
        };

        token::transfer(
            CpiContext::new(self.token_program.to_account_info(), accounts),
            amount,
        )
    }
}

impl CompleteContribution<'_> {
    pub fn give_reward_token_to_participants(&self) -> Result<()> {
        let reward_token = self.contribution_account.reward_token;
        // TODO: support multiple participants
        let participant_key = self
            .contribution_account
            .paticipants
            .iter()
            .find(|&p| p == self.participant_account.key)
            .ok_or(ContributionError::InvalidParticipant)
            .unwrap();
        let amount = reward_token.amount;

        // TODO: add bump
        let contribution_pda_seeds = &[
            self.proposer.to_account_info().key.as_ref(),
            b"contribution",
        ];

        let ix = spl_token::instruction::transfer(
            &spl_token::ID,
            self.contribution_pda_associated_token_account
                .to_account_info()
                .key,
            participant_key,
            self.contribution_pda_associated_token_account
                .to_account_info()
                .key,
            &[self
                .contribution_pda_associated_token_account
                .to_account_info()
                .key],
            amount,
        )?;

        solana_program::program::invoke_signed(
            &ix,
            &[
                self.contribution_pda_associated_token_account
                    .to_account_info(), // ctx.accounts.from.clone(),
                self.participant_account.to_account_info(),
                self.contribution_pda_associated_token_account
                    .to_account_info(),
            ],
            &[contribution_pda_seeds],
        )
        .map_err(Into::into)

        // TODO: close pda token associated token
    }
}
