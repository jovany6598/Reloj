// Client

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

// Mostrar wallet y balance
console.log("My address:", pg.wallet.publicKey.toString());

const balance = await pg.connection.getBalance(pg.wallet.publicKey);

console.log(`My balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);


// ------------------------------------
// PDA de la colección de relojes
// ------------------------------------

const [coleccionPDA] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("relojes"),
    pg.wallet.publicKey.toBuffer(),
  ],
  pg.program.programId
);

console.log("Coleccion PDA:", coleccionPDA.toString());


// ------------------------------------
// 1️⃣ Crear colección
// ------------------------------------

await pg.program.methods
  .crearColeccion("Relojes Premium")
  .accounts({
    owner: pg.wallet.publicKey,
    coleccion: coleccionPDA,
    systemProgram: web3.SystemProgram.programId,
  })
  .rpc();

console.log("Colección creada");


// ------------------------------------
// 2️⃣ Agregar reloj
// ------------------------------------

await pg.program.methods
  .agregarReloj("Rolex", "Submariner", 50000)
  .accounts({
    owner: pg.wallet.publicKey,
    coleccion: coleccionPDA,
  })
  .rpc();

console.log("Reloj agregado");


// ------------------------------------
// 3️⃣ Ver relojes
// ------------------------------------

await pg.program.methods
  .verRelojes()
  .accounts({
    owner: pg.wallet.publicKey,
    coleccion: coleccionPDA,
  })
  .rpc();

console.log("Consulta de relojes enviada");


// ------------------------------------
// 4️⃣ Cambiar disponibilidad
// ------------------------------------

await pg.program.methods
  .alternarEstado("Submariner")
  .accounts({
    owner: pg.wallet.publicKey,
    coleccion: coleccionPDA,
  })
  .rpc();

console.log("Estado del reloj cambiado");


// ------------------------------------
// 5️⃣ Eliminar reloj
// ------------------------------------

await pg.program.methods
  .eliminarReloj("Submariner")
  .accounts({
    owner: pg.wallet.publicKey,
    coleccion: coleccionPDA,
  })
  .rpc();

console.log("Reloj eliminado");


// ------------------------------------
// Leer cuenta
// ------------------------------------

const cuenta = await pg.program.account.coleccionRelojes.fetch(
  coleccionPDA
);

console.log("Contenido de la colección:", cuenta);
