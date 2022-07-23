use scrypto::prelude::*;

blueprint! {
    struct NftMarket {
        
        nft_liquidity_pool: HashMap<ResourceAddress, Vault>,
    }

    impl NftMarket {
        
        pub fn new() -> ComponentAddress {

            let component = Self {
                
                nft_liquidity_pool: HashMap::new(),
                
            }
            .instantiate()
            .globalize();

            return component;
        }

        //Check if liquidity pool exists
        pub fn pool_exists(&self, address1:ResourceAddress)->bool{
            return self.nft_liquidity_pool.contains_key(&address1);
        }

        //Assert pool exists
        pub fn assert_pool_exists(&self, address1:ResourceAddress,){

            assert!(self.pool_exists(address1), "No liquidity pool exists for the given address.");
        }

         //Assert pool dosent exists
         pub fn assert_pool_dosent_exist(&self, address1:ResourceAddress,){

            assert!(self.pool_exists(address1), "A liquidity pool with the given address already exists.");
        }
        

    }

    
}