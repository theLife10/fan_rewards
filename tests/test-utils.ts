import { createMint, getOrCreateAssociatedTokenAccount, getAccount } from "@solana/spl-token";

export async function createMintAccount(provider, mintAuthority) {
    return await createMint(
        provider.connection,
        provider.wallet.payer,
        mintAuthority,
        null, // No freeze authority
        0 // Decimals
    );
}

export async function createTokenAccount(provider, mint, owner) {
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        provider.wallet.payer,
        mint,
        owner
    );
    return tokenAccount.address;
}

export async function getTokenAccountBalance(provider, tokenAccount) {
    const accountInfo = await getAccount(provider.connection, tokenAccount);
    return Number(accountInfo.amount);
}
