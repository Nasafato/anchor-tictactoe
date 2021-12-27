import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Tictactoe } from '../target/types/tictactoe';

describe('tictactoe', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Tictactoe as Program<Tictactoe>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
