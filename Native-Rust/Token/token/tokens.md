1) Token Mint 

-- what is a mint acc ? 
   Account which stores all the infomation about token data such as who is authority to create new tokens and how many toekns in the suppy and so on info 
   and token specific info that applies to all the tokens such as ew discuseed thats known as token mint account 

   token proram defined as : 
            pub struct Mint {

    Optional    // the one who created and will further increase the supply of tokens --
                // if not declared = fixed amount of token suppy 
                mint_authority : COption<pubkey()>,

                // number of tokens in the supply now --
                supply : u64,

                // number of decimals after each token eg 1 token = 5 decimals , 1 = 0.99999 + 0.00001 ; 
                decimals : u8,

                // wether initialized or not 
                is_initialized : bool,
    
    Optional    // who will able to freeez the tokens from the supply 
                // if not declared = no one control the tokens that user owns 
                freez_authority : COption<pubkey()>,
                }

    
    Every token has 1 mint account and mint address which is unique and indefied accross wallets, applications etc 
    Ex : Token A has wperyoitjkldlf...eruoiy this mint address which will only represent the mint acc 


