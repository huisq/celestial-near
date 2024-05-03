/*!
Non-Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/


use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue,
};

///////////////////////////////////////////////////////
// CONST                                             //
//////////////////////////////////////////////////////

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";
const DRAW_PRICE: u128 =      500_000_000_000_000_000_000_000; //0.5NEAR
const MINT_PRICE: u128 =    1_000_000_000_000_000_000_000_000; //1NEAR
const MINT_STORAGE_COST: u128 = 5_870_000_000_000_000_000_000;
const VAULT: &str = "tarotvault.testnet";
const MAJOR_ARCANA_CARD_URI: &str = "ipfs://bafybeifrqo4oorpn2y2l7vy5y4v4tqebvho5q5hg5rfsx2rafzng3u556q/";
const MAJOR_ARCANA_NAME: [&'static str; 22] = [
    "0 The Fool",
    "I The Magician",
    "II The High Priestess",
    "III The Empress",
    "IV The Emperor",
    "V The Hierophant",
    "VI The Lovers",
    "VII The Chariot",
    "VIII Strength",
    "IX The Hermit",
    "X The Wheel of Fortune",
    "XI Justice",
    "XII The Hanged Man",
    "XIII Death",
    "XIV Temperance",
    "XV The Devil",
    "XVI The Tower",
    "XVII The Star",
    "XVIII The Moon",
    "XIX The Sun",
    "XX Judgement",
    "XXI The World"
];

///////////////////////////////////////////////////////
// STRUCT                                            //
//////////////////////////////////////////////////////

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    pub minted: i8
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

///////////////////////////////////////////////////////
// INIT                                             //
//////////////////////////////////////////////////////

#[near_bindgen]
impl Contract {
    /// Initializes the contract owned by `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Celestial NFT".to_string(),
                symbol: "CT".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            minted: 0
        }
    }

    /// Mint a new token with ID=`token_id` belonging to `receiver_id`.
    ///
    /// Since this example implements metadata, it also requires per-token metadata to be provided
    /// in this call. `self.tokens.mint` will also require it to be Some, since
    /// `StorageKey::TokenMetadata` was provided at initialization.
    ///
    /// `self.tokens.mint` will enforce `predecessor_account_id` to equal the `owner_id` given in
    /// initialization call to `new`.
    // To-do: 
    // 1. make payment for mints
    // 2. draw function
    
    ///////////////////////////////////////////////////////
    // ENTRY FUNCTIONS                                  //
    //////////////////////////////////////////////////////
    
    #[payable]
    pub fn draw_cards() -> Vec<String>{
        let card_index = Self::rand_num(22) as usize;
        let card = MAJOR_ARCANA_NAME[card_index].to_string();
        let position = if Self::rand_num(2) == 0 {"reverse"}else{"upright"}.to_string();
        let card_uri = format!("{}{}.png", MAJOR_ARCANA_CARD_URI, card_index.to_string());
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");
        //make sure the deposit is greater than the price
        assert!(deposit >= DRAW_PRICE, "Attached deposit must be greater than or equal to the draw price: {:?}", DRAW_PRICE);
        let vault_account_id = AccountId::new_unchecked(VAULT.to_string());
        Self::pay(DRAW_PRICE, vault_account_id.clone());
        // vec[card, card_uri, position]
        vec![card, card_uri, position]
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
        card: String,
        reading: String,
        _question: String,
        position: String
    ) -> Token {
        let token_id = self.minted;
        self.minted += 1;
        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");
        //make sure the deposit is greater than the price
        assert!(deposit >= MINT_PRICE, "Attached deposit must be greater than or equal to the mint price + storage: {:?}", MINT_PRICE + MINT_STORAGE_COST);
        let vault_account_id = AccountId::new_unchecked(VAULT.to_string());
        Self::pay(MINT_PRICE, vault_account_id.clone());
        self.tokens.internal_mint(token_id.to_string(), receiver_id, Some(Self::set_token_metadata(token_id, card, reading, position)))
    }

    ///////////////////////////////////////////////////////
    // HELPER FUNCTIONS                                  //
    //////////////////////////////////////////////////////

    pub fn pay(amount: u128, to: AccountId) -> Promise {
        Promise::new(to).transfer(amount)
    }

    //random no generator, upper limit not inclusive
    pub(crate) fn rand_num(upper_limit: u128) -> u128 {
        // Here we get a first byte of a random seed
        let random_seed = *env::random_seed().get(0).unwrap() as u128;
        return random_seed % upper_limit;
    }

    //view total minted no
    pub fn get_num(&self) -> String {
        return self.minted.to_string();
    }

    fn set_token_metadata(token_id: i8, card: String, reading: String, position: String) -> TokenMetadata {
        let name = format!("Reading#{}: {} in {}", token_id.to_string(), card, position);
        let i = MAJOR_ARCANA_NAME.iter().position(|&r| r == card);
        assert!(i != None, "Card not found");
        let card_index = i.unwrap();
        let card_uri = format!("{}{}.png", MAJOR_ARCANA_CARD_URI, card_index.to_string());
        TokenMetadata {
            title: Some(name.into()),
            description: Some(reading.into()),
            media: Some(card_uri.into()),
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

//view metadata
#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

///////////////////////////////////////////////////////
// TEST                                             //
//////////////////////////////////////////////////////

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use std::collections::HashMap;
    use near_sdk::log;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_token_metadata() -> TokenMetadata {
        let name = "Reading#0: 0 The Fool in upright".to_string();
        let card_uri = format!("{}{}.png", MAJOR_ARCANA_CARD_URI, "0".to_string());
        TokenMetadata {
            title: Some(name.into()),
            description: Some("test reading".into()),
            media: Some(card_uri.into()),
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }
    
    #[test]
    fn test_get_num() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into());
        log!("minted: {}", contract.minted.to_string());
    }

    #[test]
    fn test_draw_success() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST + DRAW_PRICE)
            .predecessor_account_id(accounts(0))
            .build());

        let draw = Contract::draw_cards();
        log!("Card:{}, Position:{}", draw[0], draw[2]);
        
    }

    #[test]
    #[should_panic(expected = "Attached deposit must be greater than or equal to the draw price: 500000000000000000000000")]
    fn test_draw_failure() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST )
            .predecessor_account_id(accounts(0))
            .build());
        let draw = Contract::draw_cards();
    }

    #[test]
    fn test_mint() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST + MINT_PRICE)
            .predecessor_account_id(accounts(0))
            .build());
        
        let card = "0 The Fool".to_string();
        let reading = "test reading".to_string();
        let question = "test question".to_string();
        let position = "upright".to_string();
        let token = contract.nft_mint(accounts(0), card, reading, question, position);
        assert_eq!(token.owner_id.to_string(), accounts(0).to_string());
        assert_eq!(token.metadata.unwrap(), sample_token_metadata());
        assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
    }

    #[test]
    fn test_mint_storage_calc() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST + MINT_PRICE)
            .predecessor_account_id(accounts(0))
            .build());
        
        let card = "0 The Fool".to_string();
        let reading = "The hierophant in reverse suggests there may be non-traditional methods and unconventional approaches involved in your negotiation with your business partner. Don’t feel boxed in by the norm or what has been successful in the past. This could lead to arguments or impasses, so be patient. Respect their viewpoint while expressing yours openly. Remember, productive dialogue involves understanding and compromises. This doesn’t mean you must abandon your beliefs, but adapting to change is crucial in this situation. Be flexible and remember your mutual goals to create a win-win situation.".to_string();
        let question = "test question".to_string();
        let position = "upright".to_string();
        contract.nft_mint(accounts(0), card, reading, question, position);
        log!("storage cost: {} yoctoNear", env::storage_byte_cost()*env::storage_usage()as u128)
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST + MINT_PRICE)
            .predecessor_account_id(accounts(0))
            .build());
        
            let card = "0 The Fool".to_string();
            let reading = "test reading".to_string();
            let question = "test question".to_string();
            let position = "upright".to_string();
            let token = contract.nft_mint(accounts(0), card, reading, question, position);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_transfer(accounts(1), token.token_id.clone(), None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        if let Some(token) = contract.nft_token(token.token_id.clone()) {
            assert_eq!(token.owner_id.to_string(), accounts(1).to_string());
            assert_eq!(token.metadata.unwrap(), sample_token_metadata());
            assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
        } else {
            panic!("token not correctly created, or not found by nft_token");
        }
    }

    #[test]
    fn test_approve() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST + MINT_PRICE)
            .predecessor_account_id(accounts(0))
            .build());
                
        let card = "0 The Fool".to_string();
        let reading = "test reading".to_string();
        let question = "test question".to_string();
        let position = "upright".to_string();
        let token = contract.nft_mint(accounts(0), card, reading, question, position);

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token.token_id.clone(), accounts(1), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(contract.nft_is_approved(token.token_id.clone(), accounts(1), Some(1)));
    }

    #[test]
    fn test_revoke() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST + MINT_PRICE)
            .predecessor_account_id(accounts(0))
            .build());
           
        let card = "0 The Fool".to_string();
        let reading = "test reading".to_string();
        let question = "test question".to_string();
        let position = "upright".to_string();
        let token = contract.nft_mint(accounts(0), card, reading, question, position);

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token.token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke(token.token_id.clone(), accounts(1));
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token.token_id.clone(), accounts(1), None));
    }

    #[test]
    fn test_revoke_all() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST + MINT_PRICE)
            .predecessor_account_id(accounts(0))
            .build());
                    
        let card = "0 The Fool".to_string();
        let reading = "test reading".to_string();
        let question = "test question".to_string();
        let position = "upright".to_string();
        let token = contract.nft_mint(accounts(0), card, reading, question, position);

        // alice approves bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(150000000000000000000)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_approve(token.token_id.clone(), accounts(1), None);

        // alice revokes bob
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(0))
            .build());
        contract.nft_revoke_all(token.token_id.clone());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert!(!contract.nft_is_approved(token.token_id.clone(), accounts(1), Some(1)));
    }
}