//import * as anchor from "@coral-xyz/anchor";


import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FanRewards } from "../target/types/fan_rewards";

describe("fan_rewards", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.FanRewards as Program<FanRewards>;

    it("Mints loyalty tokens!", async () => {
        const mint = new anchor.web3.PublicKey("EhbtYnmwrVtFVekSvtQwgo1b2pehfb87XgJh5DZQZenD");
        const receiver = new anchor.web3.PublicKey("Hd3mYMZG27Q7agzZcb3Uauq9Lx9GG3uiLrFVt68dXztT");

        const tx = await program.methods
            .mintLoyaltyTokens(new anchor.BN(100)) // Mint 100 tokens
            .accounts({
                mint: mint,
                receiver: receiver,
                mintAuthority: provider.wallet.publicKey,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            })
            .rpc();

        console.log("Transaction Signature:", tx);
    });
});


