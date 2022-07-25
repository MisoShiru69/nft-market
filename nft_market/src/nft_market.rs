use scrypto::prelude::*;
use crate::liquidity_pool::*;
use crate::nft_mint::*;  

blueprint! {
    struct NftMarket {
        
        nft_liquidity_pool: HashMap<ResourceAddress, NftLiquidityPool>,
        nft_tracking1:HashMap<ResourceAddress, HashMap<NonFungibleId, Decimal>>, 
        nft_tracking2:HashMap<NonFungibleId, Decimal>,
    }

    impl NftMarket {
        
        pub fn new() -> ComponentAddress {

            let component = Self {
                
                nft_liquidity_pool: HashMap::new(),
                nft_tracking1: HashMap::new(),
                nft_tracking2: HashMap::new(),
                
                
            }
            .instantiate()
            .globalize();

            return component;
        }

        //Check if liquidity pool exists
        pub fn pool_exists(&self, address:ResourceAddress)->bool{
            return self.nft_liquidity_pool.contains_key(&address);
        }

        //Assert pool exists
        pub fn assert_pool_exists(&self, address:ResourceAddress,){

            assert!(self.pool_exists(address), "No liquidity pool exists for the given address.");
        }

         //Assert pool dosent exists
         pub fn assert_pool_dosent_exist(&self, address:ResourceAddress,){

            assert!(!self.pool_exists(address), "A liquidity pool with the
             given address already exists.");
        }
        
        //Create new liquidity pool
        //put 1 nft in bucket, from this I shold be able to get the NFT id and the resourceaddress
        pub fn new_liquidity_pool(&mut self, nft:Bucket, nft_id:NonFungibleId, price:Decimal){

            //Get the resource address from the NFT Bucket
            let address = nft.resource_address();

            info!("address of nft in Bucket{}", address);

            //Using the NFT resource address check to see if a pool already exists
            self.assert_pool_dosent_exist(address);

            //Instantiate new liquidity pool from the liquidity pool blueprint
            let liquidity_pool:ComponentAddress = NftLiquidityPool::new(address);

            //Update the Hashmap
            self.nft_liquidity_pool.insert(address, liquidity_pool.into());

            //Update the Hashmap
            self.nft_tracking2.insert(nft_id, price);

            //clone the inner Hashmap
            let hash_map = self.nft_tracking2.clone();         

            //Update the Outer Hashmap
            self.nft_tracking1.insert(address, hash_map);

            liquidity_pool.deposit(nft);

        }
        

    }

    
}