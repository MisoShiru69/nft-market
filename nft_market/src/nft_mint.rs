use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct Nft {

    #[scrypto(mutable)]
    generation: u8,
}

blueprint! {
    struct MintNft {
        
        // nft_vault: Vault,       //This is where all the minted NFTs will be stored
        // nft_resource_address: ResourceAddress,      //This is the resource address of the newly minted nft
        // nft_mint_price:Decimal,     //This is the mint price of the newly minted NFTs    
        collected_xrd:Vault,        //The money generated from selling the NFTs is stored here 
       
    }

    impl MintNft {
        
        pub fn new() -> (ComponentAddress,Bucket) {
           
            let nft_bucket: Bucket = ResourceBuilder::new_non_fungible()
                .metadata("name", "Newly Minted NFTs")
                .initial_supply([
                    (
                        NonFungibleId::from_u64(1u64),
                        Nft {
                            generation:0
                        }
                    ),
                    (
                        NonFungibleId::from_u64(2u64),
                        Nft {
                            generation:0
                        }
                    ),
                    (
                        NonFungibleId::from_u64(3u64),
                        Nft {
                            generation:0
                        }
                    ),
                    (
                        NonFungibleId::from_u64(4u64),
                        Nft {
                            generation:0
                        }
                    ),
                    (
                        NonFungibleId::from_u64(5u64),
                        Nft {
                            generation:0
                        }
                    ),
                                
                ]);
            
            // let nft_resource_address = nft_bucket.resource_address();

            let component = Self {
                
                // nft_vault: Vault::new(nft_resource_address),
                // nft_resource_address,
                // nft_mint_price: price,
                collected_xrd:Vault::new(RADIX_TOKEN),
            }
            .instantiate()
            .globalize();

            return (component, nft_bucket);
        }

    }
}