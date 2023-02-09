#[cfg(test)]
mod tests {
    use anchor_lang::prelude::Pubkey;

    use crate::states::everlastings::*;
    use crate::states::xp_assignor::*;

    #[test]
    fn test_create_xp_assignor() {
        // Given
        let assignor_key = Pubkey::new_unique();
        let assignee_key = Pubkey::new_unique();
        let contribution_key = Pubkey::new_unique();

        let xp: u32 = 10;
        let skill = TechnologyCategory::Ethereum;

        // When
        let xp_assign = XpAssign {
            xp_gains: vec![XpGains { skill, xp }],
            assignee: assignee_key,
            claimed: false,
        };

        let xp_assignor = XpAssignor::new(
            vec![assignor_key],
            contribution_key,
            vec![xp_assign.clone()],
        );

        // Then
        assert_eq!(xp_assignor.assignor_keys[0], assignor_key);
        assert_eq!(xp_assignor.contribution_key, contribution_key);
        assert_eq!(xp_assignor.xp_assigns[0], xp_assign);
        assert_eq!(xp_assignor.assignable, false);
    }
}
