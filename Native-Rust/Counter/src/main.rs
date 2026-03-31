use solana_sdk::{signature::Keypair, signer::Signer};
use anyhow::Result;
use std::io;
fn main() {
    let mut counter : i32 = 0;
    // let mut address;
    // let mut keypair;

    let keypair = Keypair::new();
    let address = keypair.pubkey();
    println!("Account Created SuccessFully .");
    println!("Address: {address}");

    // println!(" Enter an Option : ");
    // println!(" 1 Create new account ");
    // println!(" 2 import account ");

    // let mut input = String::new();
    // println!("Enter a number:");
    // io::stdin().read_line(&mut input).expect("Failed to read input");
    // let number: u8 = input.trim().parse().expect("Invalid input");

    // match number {
    //     1 => {
    //             let keypair = Keypair::new();
    //             address = keypair.pubkey();
    //             println!("Account Created SuccessFully .");
    //             println!("Address: {address}");
    //             keypair = createNew::newWallet();
    //         },
    //     2 => println!("Under Devlopment"),
    //     _ => println!("Invalid Input"),
    // }


    // println!("{counter}");


    loop{
    println!("Counter = {counter}");
    println!("Please Enter Input : \n 1) Increase Counter \n 2) Decrease Counter \n 3) Exit ");
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: u8 = input.trim().parse().expect("Invalid input");

    let signature = keypair.sign_message(counter.to_string().as_bytes());
    let is_valid_signature = signature.verify(&keypair.pubkey().to_bytes(), counter.to_string().as_bytes());
    match number {
           
        1 => {
            if is_valid_signature == true {
                counter+=1;
                println!("Transaction SuccessFull");
            }else {
                println!("Transation failed");
            }
        },
        2 => {
            if is_valid_signature == true {
                counter-=1;
                println!("Transaction SuccessFull");
            }else {
                println!("Transation failed");
            }
        },

        3 => break,
        
        _ => println!("Invalid input try again !"),

    };
        
    }
    
    // let signature = keypair.sign_message(counter.to_string().as_bytes());
    // let is_valid_signature = signature.verify(&keypair.pubkey().to_bytes(), counter.to_string().as_bytes());
    // println!("Verified: {:?}", is_valid_signature);


}
   

