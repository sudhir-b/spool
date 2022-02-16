import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Spool } from '../target/types/spool';

describe('spool', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Spool as Program<Spool>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
