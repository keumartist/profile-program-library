import * as anchor from "@project-serum/anchor";
import { assert } from "chai";
import { Profile } from '../target/types/_profile';
import { createEverlastings, generateWallet, derivePda,  MBTI , SkillTechnology, claimSocialBadgePersnalityMbti, SocialSoftSKill, claimSocialBadgeSoftSkill, claimSkillBadgePosition, SkillPosition, claimSkillBadgeTechnology } from './utils';

describe("Testing Everlastings program api", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Profile as anchor.Program<Profile>;

  beforeEach("Set timeout when testing on devent", async () =>{
    if (provider.connection.rpcEndpoint === "https://api.devnet.solana.com") {
      await new Promise(resolve => setTimeout(resolve, 7*1000));
    }
  })
  
  it("Create everlastings", async () => {
    // Given
    const newWallet = await generateWallet(provider);

    // When
    await createEverlastings(newWallet, program)

    // Then
    const pda = derivePda(newWallet.publicKey, program.programId);
    console.log("pda of everlastings :")
    console.log(pda.toBase58())

    const everlastings = await program.account.everlastings.fetch(pda); 

    console.log("Everlastings detail :");
    console.log(JSON.stringify(everlastings, null, 4));

    assert.exists(everlastings);
  });

  it("Claim soft skill on personality badge in everlastings", async () => {
    // Given
    const newWallet = await generateWallet(provider);
    const softSkill = SocialSoftSKill.Leadership

    // When
    await createEverlastings(newWallet, program);

    const pda = derivePda(newWallet.publicKey, program.programId);

    // Then
    await claimSocialBadgeSoftSkill(newWallet, newWallet.publicKey, program, { softSkill });
    
    const afterEverlastings = await program.account.everlastings.fetch(pda);
    console.log("Everlastings detail after :");
    console.log(JSON.stringify(afterEverlastings, null, 4));

    const softSkills = afterEverlastings.socialBadge.softSkill as any;
    const category = softSkills[0].category;

    assert.equal(SocialSoftSKill[softSkill].toLowerCase(), Object.keys(category)[0]);
  });

  it("Claim position on skill badge in everlastings", async () => {
    // Given
    const newWallet = await generateWallet(provider);
    const skillPosition = SkillPosition.Designer;

    // When
    await createEverlastings(newWallet, program);

    const pda = derivePda(newWallet.publicKey, program.programId);

    // Then
    await claimSkillBadgePosition(newWallet, newWallet.publicKey, program, { skillPosition });
    
    const afterEverlastings = await program.account.everlastings.fetch(pda);
    console.log("Everlastings detail after :");
    console.log(JSON.stringify(afterEverlastings, null, 4));

    const skillPositions = afterEverlastings.skillBadge.position as any;
    const category = skillPositions[0].category;

    assert.equal(SkillPosition[skillPosition].toLowerCase(), Object.keys(category)[0]);
  });

  it("Claim technology on skill badge in everlastings", async () => {
    // Given
    const newWallet = await generateWallet(provider);
    const skillTechnology = SkillTechnology.Unity;

    // When
    await createEverlastings(newWallet, program);

    const pda = derivePda(newWallet.publicKey, program.programId);

    // Then
    await claimSkillBadgeTechnology(newWallet, newWallet.publicKey, program, { skillTechnology });
    
    const afterEverlastings = await program.account.everlastings.fetch(pda);
    console.log("Everlastings detail after :");
    console.log(JSON.stringify(afterEverlastings, null, 4));

    let technologies = afterEverlastings.skillBadge.technology as any;
    let category = technologies[0].category; 

    assert.equal(SkillTechnology[skillTechnology].toLowerCase(), Object.keys(category)[0]);
  });

  it("Claim MBTI on personality badge in everlastings", async () => {
    // Given
    const newWallet = await generateWallet(provider);
    const mbti = MBTI.ENTP;

    // When
    await createEverlastings(newWallet, program);
    const pda = derivePda(newWallet.publicKey, program.programId);

    // Then
    await claimSocialBadgePersnalityMbti(newWallet, newWallet.publicKey, program, { mbti });
    
    const afterEverlastings = await program.account.everlastings.fetch(pda);
    console.log("Everlastings detail after :");
    console.log(JSON.stringify(afterEverlastings, null, 4));
    
    let personality = afterEverlastings.socialBadge.personality as any;

    assert.equal(MBTI[mbti].toLowerCase(), Object.keys(personality.mbti)[0]);
  });
});
