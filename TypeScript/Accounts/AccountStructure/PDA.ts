import { PublicKey } from "@solana/web3.js";

const programAddress = new PublicKey("11111111111111111111111111111111");

const seeds = [Buffer.from("HelloWorld")];


// while using the async call add "type" : "module" ,  in package.JSON  
const [pda, bump] = await PublicKey.findProgramAddressSync(
    seeds ,
    programAddress
);

console.log(`PDA : ${pda}`);
console.log(`Bump : ${bump}`);