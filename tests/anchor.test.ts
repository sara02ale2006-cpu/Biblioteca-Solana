import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TiendaArte } from "../target/types/tienda_arte";

describe("tienda-arte", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TiendaArte as Program<TiendaArte>;

  const arte = anchor.web3.Keypair.generate();

  it("Crear arte digital", async () => {

    await program.methods
      .crearArte(
        "Atardecer NFT",
        "Sara",
        new anchor.BN(2),
        "Arte digital estilo anime"
      )
      .accounts({
        arte: arte.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([arte])
      .rpc();

    const cuenta = await program.account.arte.fetch(arte.publicKey);

    console.log("Arte creado:", cuenta);
  });

  it("Actualizar arte", async () => {

    await program.methods
      .actualizarArte(
        "Atardecer NFT v2",
        new anchor.BN(3)
      )
      .accounts({
        arte: arte.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const cuenta = await program.account.arte.fetch(arte.publicKey);

    console.log("Arte actualizado:", cuenta);
  });

});
