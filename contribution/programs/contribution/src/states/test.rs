#[cfg(test)]
mod tests {
    use crate::states::contribution::*;
    use std::str::FromStr;

    use _profile::states::TechnologyCategroy;
    use anchor_lang::prelude::Pubkey;

    #[test]
    fn test_create_contribution() {
        // Given
        let title = "keum".to_string();
        let skill = TechnologyCategroy::UnrealEngine;
        let xp_gains = 100;
        let mut skill_gains = Vec::new();
        skill_gains.push(SkillGains {
            skill,
            xp: xp_gains,
        });

        let point_token_gains: u32 = 0;
        let proposer = Pubkey::new_unique();
        let xp_program = Pubkey::from_str("7heeP34yJJ8aWhzG72RJAiYaJX48djqhbfhBn9ccoiGf").unwrap();
        let category = ContributionCategory::Quest;

        // When
        let contribution = Contribution::new(
            title,
            skill_gains,
            point_token_gains,
            proposer,
            xp_program,
            category,
        );

        // Then
        assert_eq!(contribution.proposer, proposer);
        assert_eq!(contribution.xp_gains.skill_gains[0].skill, skill);
        assert_eq!(contribution.xp_gains.skill_gains[0].xp, xp_gains);
        assert_eq!(contribution.point_token_gains, point_token_gains);
        assert_eq!(contribution.xp_gains.program, xp_program);
        assert_eq!(contribution.is_active, true);
        assert_eq!(contribution.is_completed, false);
        assert_eq!(contribution.category, category);
    }

    #[test]
    fn test_set_participants() {
        // Given
        let title = "keum".to_string();
        let skill = TechnologyCategroy::UnrealEngine;
        let xp_gains = 100;
        let mut skill_gains = Vec::new();
        skill_gains.push(SkillGains {
            skill,
            xp: xp_gains,
        });

        let point_token_gains: u32 = 0;
        let proposer = Pubkey::new_unique();
        let xp_program = Pubkey::from_str("7heeP34yJJ8aWhzG72RJAiYaJX48djqhbfhBn9ccoiGf").unwrap();
        let category = ContributionCategory::Quest;

        let participant_one = Pubkey::new_unique();
        let participant_two = Pubkey::new_unique();

        // When
        let mut contribution = Contribution::new(
            title,
            skill_gains,
            point_token_gains,
            proposer,
            xp_program,
            category,
        );

        // Then
        contribution.set_participant(participant_one).unwrap();
        contribution.set_participant(participant_two).unwrap();

        assert_eq!(contribution.paticipants[0], participant_one);
        assert_eq!(contribution.paticipants[1], participant_two);

        contribution.set_participant(participant_one).unwrap_err();
        contribution.set_participant(participant_two).unwrap_err();
    }

    #[test]
    fn test_complete() {
        // Given
        let title = "keum".to_string();
        let skill = TechnologyCategroy::UnrealEngine;
        let xp_gains = 100;
        let mut skill_gains = Vec::new();
        skill_gains.push(SkillGains {
            skill,
            xp: xp_gains,
        });

        let point_token_gains: u32 = 0;
        let proposer = Pubkey::new_unique();
        let xp_program = Pubkey::from_str("7heeP34yJJ8aWhzG72RJAiYaJX48djqhbfhBn9ccoiGf").unwrap();
        let category = ContributionCategory::Quest;

        let participant = Pubkey::new_unique();

        // When
        let mut contribution = Contribution::new(
            title,
            skill_gains,
            point_token_gains,
            proposer,
            xp_program,
            category,
        );

        contribution.set_participant(participant).unwrap();
        contribution.complete(participant).unwrap_err();
        contribution.complete(proposer).unwrap();

        // Then
        assert_eq!(contribution.is_active, false);
        assert_eq!(contribution.is_completed, true);
    }

    #[test]
    fn test_deactivate() {
        // Given
        let title = "keum".to_string();
        let skill = TechnologyCategroy::UnrealEngine;
        let xp_gains = 100;
        let mut skill_gains = Vec::new();
        skill_gains.push(SkillGains {
            skill,
            xp: xp_gains,
        });

        let point_token_gains: u32 = 0;
        let proposer = Pubkey::new_unique();
        let xp_program = Pubkey::from_str("7heeP34yJJ8aWhzG72RJAiYaJX48djqhbfhBn9ccoiGf").unwrap();
        let category = ContributionCategory::Quest;

        let participant = Pubkey::new_unique();
        let participant_late = Pubkey::new_unique();

        // When
        let mut contribution = Contribution::new(
            title,
            skill_gains,
            point_token_gains,
            proposer,
            xp_program,
            category,
        );

        contribution.set_participant(participant).unwrap();
        contribution.deactivate(participant).unwrap_err();
        contribution.deactivate(proposer).unwrap();

        // Then
        assert_eq!(contribution.is_active, false);
        contribution.set_participant(participant_late).unwrap_err();
    }

    #[test]
    fn test_activate() {
        // Given
        let title = "keum".to_string();
        let skill = TechnologyCategroy::UnrealEngine;
        let xp_gains = 100;
        let mut skill_gains = Vec::new();
        skill_gains.push(SkillGains {
            skill,
            xp: xp_gains,
        });

        let point_token_gains: u32 = 0;
        let proposer = Pubkey::new_unique();
        let xp_program = Pubkey::from_str("7heeP34yJJ8aWhzG72RJAiYaJX48djqhbfhBn9ccoiGf").unwrap();
        let category = ContributionCategory::Quest;

        let participant = Pubkey::new_unique();
        let participant_late = Pubkey::new_unique();

        // When
        let mut contribution = Contribution::new(
            title,
            skill_gains,
            point_token_gains,
            proposer,
            xp_program,
            category,
        );

        contribution.set_participant(participant).unwrap();
        contribution.deactivate(participant).unwrap_err();
        contribution.deactivate(proposer).unwrap();
        contribution.activate(proposer).unwrap();

        // Then
        assert_eq!(contribution.is_active, true);
        contribution.set_participant(participant_late).unwrap();
        assert_eq!(
            contribution.paticipants[contribution.paticipants.len() - 1],
            participant_late
        );
    }
}
