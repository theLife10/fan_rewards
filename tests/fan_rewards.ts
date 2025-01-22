//import * as anchor from "@coral-xyz/anchor";


import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FanRewards } from "../target/types/fan_rewards";
import {createMintAccount, createTokenAccount, getTokenAccountBalance} from "./test-utils";
import {createMint} from "@solana/spl-token";
import * as assert from "node:assert";

describe("fan_rewards", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.FanRewards as Program<FanRewards>;

    // Example accounts
    let mint: anchor.web3.PublicKey;
    let receiver: anchor.web3.PublicKey;
    let mintAuthority = provider.wallet.publicKey;

    before(async () => {
        // Create a new mint account for the test
        mint = await createMintAccount(provider, mintAuthority);

        // Create a new token account for the receiver
        receiver = await createTokenAccount(provider, mint, provider.wallet.publicKey);
        console.log("Mint Public Key:", mint.toBase58());
        console.log("Receiver Token Account Public Key:", receiver.toBase58());
    });

    it("Mints loyalty tokens!", async () => {
    //    const mint = new anchor.web3.PublicKey("EhbtYnmwrVtFVekSvtQwgo1b2pehfb87XgJh5DZQZenD");
    //    const receiver = new anchor.web3.PublicKey("Hd3mYMZG27Q7agzZcb3Uauq9Lx9GG3uiLrFVt68dXztT");

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

    it("Redeems loyalty tokens", async () => {
        const amount = new anchor.BN(50); // Redeem 50 tokens

        const tx = await program.methods
            .redeemRewards(amount)
            .accounts({
                mint: mint,
                receiver: receiver,
                mintAuthority: mintAuthority,
                userAccount: receiver,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            })
            .rpc();

        console.log("Transaction Signature:", tx);

        // Verify the token balance after redemption
        const balance = await getTokenAccountBalance(provider, receiver);
        assert.equal(balance, 50, "Receiver should have 50 tokens after redemption");
    });
});


