const anchor = require('@project-serum/anchor');
const assert = require("assert");
const { SystemProgram } = anchor.web3;

describe('ecprng', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ecprng;
  const baseAccount = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount]
    });
    // console.log("Your transaction signature", tx);

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    // console.log(program.account);
    assert.ok(account.curve.a.eq(new anchor.BN(2)));
  });

  it('Is gets a num!', async () => {
    // Add your test here.
    const tx = await program.rpc.getNum({
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });
    // console.log("Your transaction signature", tx);
  });
});
