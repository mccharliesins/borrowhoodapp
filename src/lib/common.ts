import { PasskeyKit, PasskeyServer, SACClient } from "passkey-kit";
import { Account, Keypair, StrKey } from "@stellar/stellar-sdk/minimal"
import { Buffer } from "buffer";
import { basicNodeSigner } from "@stellar/stellar-sdk/minimal/contract";
import { Server } from "@stellar/stellar-sdk/minimal/rpc";

export const rpc = new Server(import.meta.env.VITE_rpcUrl);

export const mockPubkey = StrKey.encodeEd25519PublicKey(Buffer.alloc(32))
export const mockSource = new Account(mockPubkey, '0')

export const fundKeypair = new Promise<Keypair>(async (resolve) => {
    const now = new Date();

    now.setMinutes(0, 0, 0);

    const nowData = new TextEncoder().encode(now.getTime().toString());
    const hashBuffer = await crypto.subtle.digest('SHA-256', nowData);
    const keypair = Keypair.fromRawEd25519Seed(Buffer.from(hashBuffer))
    const publicKey = keypair.publicKey()

    rpc.getAccount(publicKey)
        .catch(() => rpc.requestAirdrop(publicKey))
        .catch(() => { })

    resolve(keypair)
})

// Initialize these separately without using top-level await
export let fundPubkey: string;
export let fundSigner: any;
export let account: PasskeyKit;
export let server: PasskeyServer;
export let sac: SACClient;
export let native: any;

// Set up an initialization function
export async function initializeWallet() {
    const keypair = await fundKeypair;
    fundPubkey = keypair.publicKey();
    fundSigner = basicNodeSigner(keypair, import.meta.env.VITE_networkPassphrase);
    
    account = new PasskeyKit({
        rpcUrl: import.meta.env.VITE_rpcUrl,
        networkPassphrase: import.meta.env.VITE_networkPassphrase,
        walletWasmHash: import.meta.env.VITE_walletWasmHash,
    });
    
    server = new PasskeyServer({
        rpcUrl: import.meta.env.VITE_rpcUrl,
        launchtubeUrl: import.meta.env.VITE_launchtubeUrl,
        launchtubeJwt: import.meta.env.VITE_launchtubeJwt,
        mercuryProjectName: import.meta.env.VITE_mercuryProjectName,
        mercuryUrl: import.meta.env.VITE_mercuryUrl,
        mercuryJwt: import.meta.env.VITE_mercuryJwt,
    });
    
    sac = new SACClient({
        rpcUrl: import.meta.env.VITE_rpcUrl,
        networkPassphrase: import.meta.env.VITE_networkPassphrase,
    });
    
    native = sac.getSACClient(import.meta.env.VITE_nativeContractId);
    
    return {
        fundPubkey,
        fundSigner,
        account,
        server,
        sac,
        native
    };
}