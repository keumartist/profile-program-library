#[cfg(test)]
mod tests {
    use crate::states::everlastings::*;

    fn create_everlastings() -> Everlastings {
        Everlastings::new()
    }

    #[test]
    fn test_create__profile() {
        // Given

        // When
        let everlastings = create_everlastings();

        // Then
        assert_eq!(everlastings.skill_badge.position.len(), 0);
        assert_eq!(everlastings.skill_badge.technology.len(), 0);
        assert_eq!(everlastings.social_badge.personality.mbti, None);
        assert_eq!(everlastings.social_badge.soft_skill.len(), 0);
    }

    #[test]
    fn test_create_social_badge() {
        // Given

        // When
        let social_badge = SocialBadge::new();

        // Then
        assert_eq!(social_badge.personality.mbti, None);
        assert_eq!(social_badge.soft_skill.len(), 0);
    }

    #[test]
    fn test_create_skill_badge() {
        // Given

        // When
        let skill_badge = SkillBadge::new();

        // Then
        assert_eq!(skill_badge.position.len(), 0);
        assert_eq!(skill_badge.technology.len(), 0);
    }

    #[test]
    fn test_set_social_badge_personality_mbti() {
        // Given
        let mbti = MBTI::ENFP;

        // When
        let mut everlastings = create_everlastings();
        everlastings.set_social_badge_persnality_mbti(mbti);

        // Then
        assert_eq!(everlastings.social_badge.personality.mbti, Some(mbti));
    }

    #[test]
    fn test_set_social_badge_soft_skill() {
        // Given
        let soft_skill = SoftSkillCategory::Communication;

        // When
        let mut everlastings = create_everlastings();
        everlastings.set_social_badge_soft_skill(soft_skill);

        // Then
        assert_eq!(everlastings.social_badge.soft_skill[0].category, soft_skill);
        assert_eq!(everlastings.social_badge.soft_skill[0].xp, 0);
    }

    #[test]
    fn test_set_skill_badge_position() {
        // Given
        let skill_position = SkillPositionCategory::Designer;

        // When
        let mut everlastings = create_everlastings();
        everlastings.set_skill_badge_position(skill_position);

        // Then
        assert_eq!(
            everlastings.skill_badge.position[0].category,
            skill_position
        );
    }

    #[test]
    fn test_set_skill_badge_technology() {
        // Given
        let technology = TechnologyCategory::UnrealEngine;

        // When
        let mut everlastings = create_everlastings();
        everlastings.set_skill_badge_technology(technology);

        // Then
        assert_eq!(everlastings.skill_badge.technology[0].category, technology);
        assert_eq!(everlastings.skill_badge.technology[0].xp, 0);
    }

    #[test]
    fn test_assign_skill_xp() {
        // Given
        let technology = TechnologyCategory::UnrealEngine;
        let xp = 10;

        // When
        let mut everlastings = create_everlastings();
        everlastings.set_skill_badge_technology(technology);
        let _ = everlastings.assign_skill_xp(Vec::from([Technology {
            category: technology,
            xp,
        }]));

        // Then
        let technology_on_everlastings = everlastings
            .skill_badge
            .technology
            .iter()
            .find(|tech| tech.category == technology);

        assert_ne!(technology_on_everlastings, None);
        assert_eq!(technology_on_everlastings.unwrap().xp, xp);
    }
}
