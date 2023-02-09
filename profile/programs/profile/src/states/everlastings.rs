use anchor_lang::prelude::*;

#[account]
pub struct Everlastings {
    pub skill_badge: SkillBadge,
    pub social_badge: SocialBadge,
}

impl Everlastings {
    pub fn initialize(&mut self) {
        self.skill_badge = SkillBadge::new();
        self.social_badge = SocialBadge::new();
    }
}

impl Everlastings {
    pub fn set_social_badge_persnality_mbti(&mut self, mbti: MBTI) {
        self.social_badge.personality = Personality { mbti: Some(mbti) };
    }

    pub fn set_social_badge_soft_skill(&mut self, category: SoftSkillCategory) {
        let soft_skills = &mut self.social_badge.soft_skill;

        // TODO: need logic
        let soft_skill = SoftSkill { category, xp: 0 };
        soft_skills.push(soft_skill);
    }
}

impl Everlastings {
    pub fn set_skill_badge_position(&mut self, category: SkillPositionCategory) {
        let skill_positions = &mut self.skill_badge.position;

        // TODO: need logic
        let skill_position = Position { category };
        skill_positions.push(skill_position);
    }

    pub fn set_skill_badge_technology(&mut self, category: TechnologyCategory) {
        let technologies = &mut self.skill_badge.technology;

        // TODO: need logic
        let technology = Technology { category, xp: 0 };
        technologies.push(technology);
    }

    pub fn assign_skill_xp(&mut self, technologies: Vec<Technology>) -> Result<()> {
        self.skill_badge.assign_xp(technologies)
    }
}

// For test
impl Everlastings {
    pub fn new() -> Self {
        Self {
            skill_badge: SkillBadge::new(),
            social_badge: SocialBadge::new(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct SkillBadge {
    pub position: Vec<Position>,
    pub technology: Vec<Technology>,
}

impl SkillBadge {
    pub fn new() -> Self {
        SkillBadge {
            position: Vec::new(),
            technology: Vec::new(),
        }
    }

    fn assign_xp(&mut self, technologies: Vec<Technology>) -> Result<()> {
        for tech in technologies.iter() {
            let skill_badge_technology = self
                .technology
                .iter_mut()
                .find(|t| t.category == tech.category);

            if skill_badge_technology != None {
                let technology = skill_badge_technology.unwrap();
                technology.xp = tech.xp;
            }
        }

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub struct Position {
    pub category: SkillPositionCategory,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub struct Technology {
    pub category: TechnologyCategory,
    pub xp: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub enum SkillPositionCategory {
    Designer,
    Developer,
    Creator,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub enum TechnologyCategory {
    UnrealEngine,
    Unity,
    Solana,
    Ethereum,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub struct SocialBadge {
    pub personality: Personality,
    pub soft_skill: Vec<SoftSkill>,
}

impl SocialBadge {
    pub fn new() -> Self {
        SocialBadge {
            personality: Personality { mbti: None },
            soft_skill: Vec::new(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub struct Personality {
    pub mbti: Option<MBTI>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub struct SoftSkill {
    pub category: SoftSkillCategory,
    pub xp: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub enum MBTI {
    INFP,
    ENFP,
    INFJ,
    ENFJ,
    INTJ,
    ENTJ,
    INTP,
    ENTP,
    ISFP,
    ESFP,
    ISTP,
    ESTP,
    ISFJ,
    ESFJ,
    ISTJ,
    ESTJ,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, Copy)]
pub enum SoftSkillCategory {
    Leadership,
    Communication,
    Language,
}
