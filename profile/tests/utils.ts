import * as anchor from '@project-serum/anchor';
import { SystemProgram, PublicKey } from '@solana/web3.js';
import { Profile } from '../target/types/_profile';

export const generateWallet = async (provider: anchor.Provider) => {
    const keypair = anchor.web3.Keypair.generate();

    await provider.connection.requestAirdrop(
        keypair.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
    );
    
    console.log("Waiting to get airdrop sol");
    await new Promise(resolve => setTimeout(resolve, 2 * 1000));
    return keypair;
}

export const derivePda = (walletPubkey: anchor.web3.PublicKey, programId: anchor.web3.PublicKey) => {
    const [pda, ] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            walletPubkey.toBuffer(),
            Buffer.from("badge"),
        ],
        programId
    );

    return pda; 
}

export const createEverlastings = async (wallet: anchor.web3.Keypair, program: anchor.Program<Profile>) => {
    console.log("Creating  profile ...");

    const pda = derivePda(wallet.publicKey, program.programId);

    const tx = await program.methods.createEverlastings().accounts({
      everlastingsAccount: pda,
      wallet: wallet.publicKey,
      systemProgram: SystemProgram.programId 
    })
    .signers([wallet])
    .rpc();

    console.log(" profile created!")
    console.log("Transaction hash : ", tx);
};

export enum MBTI {
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

export enum SkillPosition {
    Designer,
    Developer,
    Creator,
}

export enum SkillTechnology {
    UnrealEngine,
    Unity,
    Solana,
    Ethereum,
}

export enum SocialSoftSKill {
    Leadership,
    Communication,
    Language,
}

export const claimSocialBadgePersnalityMbti = async (wallet: anchor.web3.Keypair, recipient: PublicKey, program: anchor.Program<Profile>, params: { mbti: MBTI }) => {
    console.log("Claiming MBTI on personality badge in Everlastings ...");

    const pda = derivePda(recipient, program.programId);
    const tx = await program.methods.claimSocialBadgePersnalityMbti(params.mbti).accounts({
      everlastingsAccount: pda,
      wallet: wallet.publicKey,
      systemProgram: SystemProgram.programId 
    })
    .signers([wallet])
    .rpc();

    console.log("Transaction hash : ", tx);
};


export const claimSkillBadgePosition = async (signer: anchor.web3.Keypair, recipient: PublicKey, program: anchor.Program<Profile>, params: { skillPosition: SkillPosition }) => {
    console.log("Claiming skill position in Everlastings ...");

    const pda = derivePda(recipient, program.programId);

    const tx = await program.methods.claimSkillBadgePosition(params.skillPosition).accounts({
      everlastingsAccount: pda,
      wallet: signer.publicKey,
      systemProgram: SystemProgram.programId 
    })
    .signers([signer])
    .rpc();

    console.log("Transaction hash : ", tx);
};

export const claimSkillBadgeTechnology = async (signer: anchor.web3.Keypair, recipient: PublicKey, program: anchor.Program<Profile>, params: { skillTechnology: SkillTechnology }) => {
    console.log("Claiming skill technology in Everlastings ...");

    const pda = derivePda(recipient, program.programId);

    const tx = await program.methods.claimSkillBadgeTechnology(params.skillTechnology).accounts({
      everlastingsAccount: pda,
      wallet: signer.publicKey,
      systemProgram: SystemProgram.programId 
    })
    .signers([signer])
    .rpc();

    console.log("Transaction hash : ", tx);
};

export const claimSocialBadgeSoftSkill = async (signer: anchor.web3.Keypair, recipient: PublicKey, program: anchor.Program<Profile>, params: { softSkill: SocialSoftSKill }) => {
    console.log("Claiming soft skill in Everlastings ...");

    const pda = derivePda(recipient, program.programId);

    const tx = await program.methods.claimSocialBadgeSoftSkill(params.softSkill).accounts({
      everlastingsAccount: pda,
      wallet: signer.publicKey,
      systemProgram: SystemProgram.programId 
    })
    .signers([signer])
    .rpc();

    console.log("Transaction hash : ", tx);
};
