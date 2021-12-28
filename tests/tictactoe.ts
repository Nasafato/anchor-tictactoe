import * as anchor from "@project-serum/anchor";
import assert from "assert";

describe("tictactoe", () => {
  const program = anchor.workspace.Tictactoe;
  const provider = anchor.Provider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const baseAccount = anchor.web3.Keypair.generate();

  const player2 = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    let tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    console.log("Your transaction signature", tx);
  });

  it("creates a game", async () => {
    let account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    console.log("Trying to create game");
    let tx = await program.rpc.createGame({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  });

  it("fails to join a game", async () => {
    try {
      await program.rpc.joinGame(new anchor.BN(3), {
        accounts: {
          joiner: player2.publicKey,
          baseAccount: baseAccount.publicKey,
        },
        signers: [player2],
      });
      assert.ok(false);
    } catch (err) {
      assert.equal(err.toString(), "Game not found");
    }
  });

  it("joins a game", async () => {
    await program.rpc.joinGame(new anchor.BN(0), {
      accounts: {
        joiner: player2.publicKey,
        baseAccount: baseAccount.publicKey,
      },
      signers: [player2],
    });
  });

  it("plays a move", async () => {
    await program.rpc.playMove(new anchor.BN(0), new anchor.BN(0), {
      accounts: {
        player: provider.wallet.publicKey,
        baseAccount: baseAccount.publicKey,
      },
    });
  });

  it("fails to play a move", async () => {
    try {
      await program.rpc.playMove(new anchor.BN(0), new anchor.BN(0), {
        accounts: {
          player: provider.wallet.publicKey,
          baseAccount: baseAccount.publicKey,
        },
      });
      assert.ok(false);
    } catch (err) {
      assert.equal(err.toString(), "Not the player's turn");
    }
  });

  it("fails to play a move again", async () => {
    try {
      await program.rpc.playMove(new anchor.BN(0), new anchor.BN(0), {
        accounts: {
          player: player2.publicKey,
          baseAccount: baseAccount.publicKey,
        },
        signers: [player2],
      });
      assert.ok(false);
    } catch (err) {
      assert.equal(err.toString(), "You cannot make that move");
    }
  });

  it("Game finishes", async () => {
    const plays = [
      [1, 1],
      [2, 0],
      [4, 1],
      [3, 0],
      [7, 1],
    ];
    const wallets = [provider.wallet, player2];
    for (const [position, walletIndex] of plays) {
      const wallet = wallets[walletIndex];
      await program.rpc.playMove(new anchor.BN(0), new anchor.BN(position), {
        accounts: {
          player: wallet.publicKey,
          baseAccount: baseAccount.publicKey,
        },
        ...(walletIndex === 1 && { signers: [wallet] }),
      });
    }
  });

  it("cannot play moves after game is finished", async () => {
    try {
      await program.rpc.playMove(new anchor.BN(0), new anchor.BN(8), {
        accounts: {
          player: provider.wallet.publicKey,
          baseAccount: baseAccount.publicKey,
        },
      });
      assert.ok(false);
    } catch (err) {
      assert.equal(err.toString(), "Game has finished");
    }
  });
});
