1) Token Mint 

-- what is a mint acc ? 
   Account which stores all the infomation about token data such as who is authority to create new tokens and how many toekns in the suppy and so on info 
   and token specific info that applies to all the tokens such as ew discuseed thats known as token mint account 
   owned by token program

   token mint defined as : 
            pub struct Mint {

    Optional    // the one who created and will further increase the supply of tokens --
                // if not declared = fixed amount of token suppy 
                mint_authority : COption<pubkey>,

                // number of tokens in the supply now --
                supply : u64,

                // number of decimals after each token eg 1 token = 5 decimals , 1 = 0.99999 + 0.00001 ; 
                decimals : u8,

                // wether initialized or not 
                is_initialized : bool,
    
    Optional    // who will able to freeez the tokens from the supply 
                // if not declared = no one control the tokens that user owns 
                freez_authority : COption<pubkey>,
                }

    
    Every token has 1 mint account and mint address which is unique and indefied accross wallets, applications etc 
    Ex : Token A has wperyoitjkldlf...eruoiy this mint address which will only represent the mint acc 


2) Token Account 

-- what is token account ? 
   token account stores individual token mint authority and token owner, which token is this , what amount of token have , who is the owner of the tokens means the wallet pubkey, delegate alternate autority to spend this tokens , these kind of stuff (Details) stored by the token account 

token account : 
                pub struct Account {
                    // the mint associated with this token account     
                    mint : Pubkey,

                    // the pubkey of owner (the authority which can transfer mint and burn tokens here is "token program")
                    owner : Pubkey, 

                    // the amount of tokens this account holding now 
                    amount : u64,

                    // the account which delegated to this account means the one who can grant permission to other wallet or program to spent tokens from this account 
                    delegate : Pubkey,
    q               
                    // state of account is initialized , uninitialized or freeezed basically account state is enum 
                    state  : AccountState, 

                    // represent is it native sol tokens or the spl tokens if none then its spl token and if some<Pubkey> its sol token 
                    is_native : COption<Pubkey> 

                    // the amount of tokens can be spend by other delegated authotities 
                    delegate_amount : u64,

                    // optional authority to close the account 
    Optional        close_authority : COption<Pubkey>    

                }

        Each token account is only bounded to only one token mint which means one token account can hold only one kind of token and the token is identified from the mint feild .


3) Associated Token Account 

-- what is associated token account ? 
   Associated token account is the deterministic way to store user tokens using programId userId and mintId (Id == Pubkey) 
   acccount will still the token account owned by token program not by the associated token program 
   ///(which means account will not be owned by the program in which we will derive the account) 
   Only token accounts which are created by associated token program are known as associated token accounts
   Associated token program is a way to create a token account at a standard , deterministic address . 

   Associated token account derivation (creation) : 

        fn pub ata (
            wallet_address : &Pubkey,
            token_mint_addres : &Pubkey,
            program_id : &Pubkey,
            token_program_id : &pubkey
        ) ->(Pubkey,u8){
            Pubkey::find_program_address(
                &[
                    &wallet_address.to_le_bytes(),       /// Owner's public key 
                    &program_id.to_le_bytes(),           /// token program 
                    &token_mint_address.to_le_bytes(),   /// token mind address 
                ],
                program_id,  // associated token program Id 
            )
        }


        for any wallet,token and mint add combination there is only one ATA . x,y,z == zyx will always 