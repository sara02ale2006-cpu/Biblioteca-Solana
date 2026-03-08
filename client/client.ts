export const crearArte = async (
  titulo: string,
  autor: string,
  precio: number,
  descripcion: string
) => {

  const tx = await program.methods
    .crearArte(titulo, autor, precio, descripcion)
    .accounts({
      arte: arteKeypair.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([arteKeypair])
    .rpc();

  console.log("Arte creado:", tx);
};
