import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Lottery } from "../target/types/lottery";
import * as web3 from "@solana/web3.js";
import { BN } from "bn.js";
import {
  createMint, 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  getMint, 
  getAccount, 
  Mint,
  TokenAccountNotFoundError,
  Account, 
  getAssociatedTokenAddressSync,
  createAccount,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { readFileSync } from "fs";


describe("lottery", async() => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Lottery as Program<Lottery>;

  const initializer = loadKeypairFromFile("/home/odmin/.config/solana/id.json");
  const withdrawKeypair = loadKeypairFromFile("/home/odmin/.config/solana/id.json");
  const poolKeypair = loadKeypairFromFile("/home/odmin/test_key_2.json");

  let gameToken = new web3.PublicKey("HYrpV9vBvTzYRMoGfBkzjsxNQcJjWLqNUKwf5C7oy8uf");
  
  let poolATA = await getOrCreateAssociatedTokenAccount(provider.connection, poolKeypair, gameToken, poolKeypair.publicKey);
  let withdrawATA = await getOrCreateAssociatedTokenAccount(provider.connection, withdrawKeypair, gameToken, withdrawKeypair.publicKey);
  let [globalPDA] = await web3.PublicKey.findProgramAddressSync([Buffer.from("GLOBAL_SETTING_SEED"), initializer.publicKey.toBuffer()], program.programId);
  let [lotteryKeyInfoPDA] = await web3.PublicKey.findProgramAddressSync([Buffer.from("LOTTERY_PDAKEY_INFO")], program.programId);
  
  let time_frame = [1,3,6,12,24,168,720,2160,4320,8640];
  let ticket_price = [1,2,3,4,5,6,7,8,9,10];
  let max_tickets = [56,56,56,56,56,56,56,56,56,56];
  let dev_fees = [10,9,8,7,6,5,4,3,2,1];


      // console.log("*****")
      // const txHash = await program.methods.initialize()
      // .accounts({
      //   globalAccount: globalPDA,
      //   poolTokenAccount: poolATA.address,
      //   lotteryPdakeyInfo: lotteryKeyInfoPDA,
      //   withdrawTokenAccount: withdrawATA.address,
      //   systemProgram: web3.SystemProgram.programId
      // })
      // .signers([initializer])
      // .rpc()
      // .catch((error) => {
      //   console.log("Transaction Error", error);
      // });
      const globalAccount = await program.account.globalAccount.fetch(globalPDA);
    //  console.log(globalAccount) 
// if (globalAccount.isInitialized ==1){
//     for (let i=0;i<10;i++){
//           let lotteryPDA = get_pda([Buffer.from("LOTTERY_INFO_SEED"), initializer.publicKey.toBuffer(), new Uint8Array([i])], program.programId)

//           let time_frame_index = i;
//           await program.methods.createLottery(
//             i,
//             time_frame_index, 
//             new BN(time_frame[i]),     
//             ticket_price[i],           
//             new BN(max_tickets[i]),
//             dev_fees[i]  
//           )
//           .accounts({
//             admin: initializer.publicKey,
//             lottery: lotteryPDA,
//             lotteryPdakeyInfo: lotteryKeyInfoPDA,
//             systemProgram: web3.SystemProgram.programId
//           })
//           .signers([initializer])
//           .rpc()
//           .catch((error)=>{console.log(error)});
//         }
//       }
//       let lotteryList = await program.account.lottery.all();
//       console.log(lotteryList,"lottery LIst");
/********* Buy Ticket  **********/
// const buyer1 = loadKeypairFromFile("/home/odmin/test_key_2.json");  
// let lotteryOnePDA = get_pda([Buffer.from("LOTTERY_INFO_SEED"), initializer.publicKey.toBuffer(), new Uint8Array([5])], program.programId);
// let buyer1PDA = get_pda([Buffer.from("USER_INFO_SEED"), buyer1.publicKey.toBuffer()], program.programId);
// let buyer1ATA = await getOrCreateAssociatedTokenAccount(provider.connection, buyer1, gameToken, buyer1.publicKey);
// let buyer1Money = await getAccount(provider.connection, buyer1ATA.address);
// let count = 5;
// console.log(buyer1Money.amount.toString());
//   let lotteryData = await program.account.lottery.fetch(lotteryOnePDA);
//   await program.methods.buyTicket(count)
//     .accounts({
//         buyer: buyer1.publicKey,
//         globalAccount: globalPDA,
//         user: buyer1PDA,
//         lottery: lotteryOnePDA,
//         poolTokenAccount: poolATA.address,
//         buyerTokenAccount: buyer1ATA.address,
//         tokenProgram: TOKEN_PROGRAM_ID
//     })
//     .signers([buyer1])
//     .rpc()
//     .catch((error) =>{console.log(error)});
//     let lotteryOnePool = await program.account.lottery.fetch(lotteryOnePDA);
//     console.log(lotteryOnePool.realPoolAmount);
//     console.log(buyer1Money.amount.toString(),"***********balance********");
//     let userData = await program.account.user.fetch(buyer1PDA);
//     console.log(userData,"userData");



/**************** Check User Ticket *************/
  const user = loadKeypairFromFile("/home/odmin/test_key_2.json");
  let [userPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("USER_INFO_SEED"), user.publicKey.toBuffer()], program.programId);
  let lotteryPDA = get_pda([Buffer.from("LOTTERY_INFO_SEED"), initializer.publicKey.toBuffer(), new Uint8Array([4])], program.programId);

  const result = await program.methods.getUserTicket()
    .accounts({
      signer: user.publicKey,
      user: userPDA,
      lottery: lotteryPDA
    })
    .signers([user])
    .rpc()
    .catch((error) => {
      console.log(error);
    });
  console.log(result,"result");

});


export const get_pda = ( seeds: (Buffer | Uint8Array)[], programId: anchor.web3.PublicKey): anchor.web3.PublicKey => {
  const [pdaKey] = anchor.web3.PublicKey.findProgramAddressSync(seeds, programId);
  return pdaKey;
}

function loadKeypairFromFile(filePath: string): web3.Keypair {
  const keypairData = JSON.parse(readFileSync(filePath, 'utf-8'));
  return web3.Keypair.fromSecretKey(new Uint8Array(keypairData));
}

export const addSols = async ( provider: anchor.Provider, wallet: anchor.web3.PublicKey, amount = 1 * anchor.web3.LAMPORTS_PER_SOL) => {
  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(wallet, amount),"confirmed"
  );
};
