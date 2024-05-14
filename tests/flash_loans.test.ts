import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { FlashLoans } from '../target/types/flash_loans';
import { Borrower } from '../target/types/borrower';
import * as assert from "assert";
const {SystemProgram} = anchor.web3;

describe("close_bundled_position", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const flashLoanProgram = anchor.workspace.FlashLoans as Program<FlashLoans>;
    const borrowerProgram = anchor.workspace.Borrower as Program<Borrower>;

    const lender = anchor.web3.Keypair.generate();
    const borrower = anchor.web3.Keypair.generate();

    it("successfully return flash loan after performed the operation", async () => {
          // Airdrop SOL to lender for testing
      await provider.connection.confirmTransaction(
          await provider.connection.requestAirdrop(lender.publicKey, 1000000000),
          "confirmed"
      );

      // Fetch the lender's balance before the flash loan
      const lenderBalanceBefore = await provider.connection.getBalance(lender.publicKey);

      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(borrower.publicKey, 100000000),
        "confirmed"
      );

      const amount = new anchor.BN(100000000);

      await flashLoanProgram.methods.executeFlashLoan(amount)
        .accounts({
          lender: lender.publicKey,
          borrower: borrower.publicKey,
          borrowerProgram: borrowerProgram.programId,
          systemProgram: SystemProgram.programId,
        })
        .signers([lender, borrower])
        .rpc();

      // Fetch the lender's balance after the flash loan
      const lenderBalanceAfter = await provider.connection.getBalance(lender.publicKey);
      
      console.log(`Lender balance after: ${lenderBalanceAfter}`);
      assert.strictEqual(lenderBalanceBefore, lenderBalanceAfter, "Lender balance should be unchanged after the flash loan");
      console.log("Flash loan executed successfully v1");
    });
})