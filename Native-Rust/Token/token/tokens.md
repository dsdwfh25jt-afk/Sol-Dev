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