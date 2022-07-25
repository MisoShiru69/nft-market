use scrypto::prelude::*;

blueprint! {
    struct NftLiquidityPool {
              
        nft_vault:Vault,

    }

    impl NftLiquidityPool {
        
        pub fn new(address:ResourceAddress) -> ComponentAddress {          

            let nft_liquidity_pool = Self {
                
                nft_vault:Vault::new(address),
            }
            .instantiate()
            .globalize();

            return nft_liquidity_pool;
        }


        pub fn deposit(&mut self, deposit:Bucket){
            self.nft_vault.put(deposit);
        }

    }
}