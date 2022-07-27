use scrypto::prelude::*;
use crate::liquidity_pool::*;
use crate::nft_mint::*;  

blueprint! {
    struct NftMarket {
        
        //Hashmap maps NFT resource address to the corresponding liquidity pool
        resouce_liquidity_tracker: HashMap<ResourceAddress, NftLiquidityPool>,

        //Hashmap maps NFT source address to Hashmap containing NFT id and sell price
        resource_hash_tracker:HashMap<ResourceAddress, HashMap<NonFungibleId, Decimal>>,
        
        //Hashmap maps NFT id to sell price
        id_price_tracker:HashMap<NonFungibleId, Decimal>,
    }

    impl NftMarket {
        
        pub fn new() -> ComponentAddress {

            let component = Self {
                
                resouce_liquidity_tracker: HashMap::new(),
                resource_hash_tracker: HashMap::new(),
                id_price_tracker: HashMap::new(),
                
                
            }
            .instantiate()
            .globalize();

            return component;
        }

        //Check if liquidity pool exists
        pub fn pool_exists(&self, address:ResourceAddress)->bool{
            return self.resouce_liquidity_tracker.contains_key(&address);
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

        fn new_liquidity_pool(&mut self, nft_resouce_address:ResourceAddress){

            //Instantiate new liquidity pool from the liquidity pool blueprint
            let new_liquidity_pool_address:ComponentAddress = NftLiquidityPool::new(nft_resouce_address);

            //Get the liquidity pool component
            let new_liquidity_pool_component:NftLiquidityPool = new_liquidity_pool_address.into();

            //Update the liquidity pool tracker Hashmap - maps resouce address to liquidity pool component
            self.resouce_liquidity_tracker.insert(nft_resouce_address, new_liquidity_pool_component);

        }

        fn update_tracker_hashmaps(&mut self, nft_resource_address:ResourceAddress, nft_id:NonFungibleId, price:Decimal) {

            //Update NFT id and price Hashmap - maps NFT id to sell price
            self.id_price_tracker.insert(nft_id, price);

            //clone NFT id and price Hashmap 
            let id_price_tracker_clone = self.id_price_tracker.clone();         

            //Update the outer Hashmap
            self.resource_hash_tracker.insert(nft_resource_address, id_price_tracker_clone);

        }

        pub fn add_liquidity(&mut self, mut nft_bucket:Bucket, nft_id:NonFungibleId, price:Decimal)->Bucket{

            //Put all the NFTs in a bucket and take the one with the corresponding NFT ID out.
            let nft_bucket_id:Bucket = nft_bucket.take_non_fungible(&nft_id);

            //Check to see if liquidity pool already exisits in the Hashmap<ResouceAddress, NftLiquidityPool>
            let existing_liquidity_pool: Option<&NftLiquidityPool> = self.resouce_liquidity_tracker.get(&nft_bucket_id.resource_address());
            
            match existing_liquidity_pool {

                Some (existing_liquidity_pool) => { //If there is a match, this means it has already been created.
                    
                    info!("[DEX Add Liquidity]: Pool for {:?} already exists. Adding liquidity directly.", nft_bucket_id.resource_address());

                    self.update_tracker_hashmaps(nft_bucket_id.resource_address(), nft_id, price);

                    existing_liquidity_pool.deposit(nft_bucket_id);

                    return nft_bucket;
                
                }
                None => { //If there is no match, this means we need to create a new liquidity pool

                    info!("[DEX Add Liquidity]: Pool for {:?} doesn't exist. Creating a new one.", nft_bucket_id.resource_address());

                    //Create new liquidity pool
                    self.new_liquidity_pool(nft_bucket.resource_address());
                    self.update_tracker_hashmaps(nft_bucket_id.resource_address(), nft_id, price);

                    let existing_liquidity_pool: Option<&NftLiquidityPool> = self.resouce_liquidity_tracker.get(&nft_bucket_id.resource_address());
                    
                    match existing_liquidity_pool {

                        Some (existing_liquidity_pool) => {
                            existing_liquidity_pool.deposit(nft_bucket_id);

                            return nft_bucket;
                        }
                        None => {
                            info!("Something went wrong");
                            return nft_bucket;
                        }
                    }
                    
                }
            }
        }
    }

}