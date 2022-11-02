pub use art_gobblers::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod art_gobblers {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ArtGobblers was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"gobblerId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"nft\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ArtGobbled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"gobblerId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"GobblerClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"gobblerId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"price\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GobblerPurchased\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"numGobblers\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"lastRevealedId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GobblersRevealed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"newGooBalance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GooBalanceUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"gobblerId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256[]\",\"name\":\"burnedGobblerIds\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LegendaryGobblerMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newRandProvider\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RandProviderUpgraded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"randomness\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RandomnessFulfilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toBeRevealed\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RandomnessRequested\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"lastMintedGobblerId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"numGobblersEach\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ReservedGobblersMinted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ARTGOBBLERS_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct ArtGobblers<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ArtGobblers<M> {
        fn clone(&self) -> Self {
            ArtGobblers(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ArtGobblers<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ArtGobblers<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ArtGobblers))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ArtGobblers<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ARTGOBBLERS_ABI.clone(), client).into()
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ArtGobbled` event"]
        pub fn art_gobbled_filter(&self) -> ethers::contract::builders::Event<M, ArtGobbledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `GobblerClaimed` event"]
        pub fn gobbler_claimed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GobblerClaimedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `GobblerPurchased` event"]
        pub fn gobbler_purchased_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GobblerPurchasedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `GobblersRevealed` event"]
        pub fn gobblers_revealed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GobblersRevealedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `GooBalanceUpdated` event"]
        pub fn goo_balance_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, GooBalanceUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LegendaryGobblerMinted` event"]
        pub fn legendary_gobbler_minted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LegendaryGobblerMintedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RandProviderUpgraded` event"]
        pub fn rand_provider_upgraded_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RandProviderUpgradedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RandomnessFulfilled` event"]
        pub fn randomness_fulfilled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RandomnessFulfilledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RandomnessRequested` event"]
        pub fn randomness_requested_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RandomnessRequestedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReservedGobblersMinted` event"]
        pub fn reserved_gobblers_minted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReservedGobblersMintedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ArtGobblersEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ArtGobblers<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ArtGobbled",
        abi = "ArtGobbled(address,uint256,address,uint256)"
    )]
    pub struct ArtGobbledFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gobbler_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub nft: ethers::core::types::Address,
        pub id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "GobblerClaimed", abi = "GobblerClaimed(address,uint256)")]
    pub struct GobblerClaimedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gobbler_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "GobblerPurchased",
        abi = "GobblerPurchased(address,uint256,uint256)"
    )]
    pub struct GobblerPurchasedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gobbler_id: ethers::core::types::U256,
        pub price: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "GobblersRevealed",
        abi = "GobblersRevealed(address,uint256,uint256)"
    )]
    pub struct GobblersRevealedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub num_gobblers: ethers::core::types::U256,
        pub last_revealed_id: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "GooBalanceUpdated", abi = "GooBalanceUpdated(address,uint256)")]
    pub struct GooBalanceUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub new_goo_balance: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "LegendaryGobblerMinted",
        abi = "LegendaryGobblerMinted(address,uint256,uint256[])"
    )]
    pub struct LegendaryGobblerMintedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub gobbler_id: ethers::core::types::U256,
        pub burned_gobbler_ids: Vec<ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "RandProviderUpgraded",
        abi = "RandProviderUpgraded(address,address)"
    )]
    pub struct RandProviderUpgradedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_rand_provider: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "RandomnessFulfilled", abi = "RandomnessFulfilled(uint256)")]
    pub struct RandomnessFulfilledFilter {
        pub randomness: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "RandomnessRequested",
        abi = "RandomnessRequested(address,uint256)"
    )]
    pub struct RandomnessRequestedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub to_be_revealed: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ReservedGobblersMinted",
        abi = "ReservedGobblersMinted(address,uint256,uint256)"
    )]
    pub struct ReservedGobblersMintedFilter {
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub last_minted_gobbler_id: ethers::core::types::U256,
        pub num_gobblers_each: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ArtGobblersEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        ArtGobbledFilter(ArtGobbledFilter),
        GobblerClaimedFilter(GobblerClaimedFilter),
        GobblerPurchasedFilter(GobblerPurchasedFilter),
        GobblersRevealedFilter(GobblersRevealedFilter),
        GooBalanceUpdatedFilter(GooBalanceUpdatedFilter),
        LegendaryGobblerMintedFilter(LegendaryGobblerMintedFilter),
        RandProviderUpgradedFilter(RandProviderUpgradedFilter),
        RandomnessFulfilledFilter(RandomnessFulfilledFilter),
        RandomnessRequestedFilter(RandomnessRequestedFilter),
        ReservedGobblersMintedFilter(ReservedGobblersMintedFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ArtGobblersEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = ArtGobbledFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::ArtGobbledFilter(decoded));
            }
            if let Ok(decoded) = GobblerClaimedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::GobblerClaimedFilter(decoded));
            }
            if let Ok(decoded) = GobblerPurchasedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::GobblerPurchasedFilter(decoded));
            }
            if let Ok(decoded) = GobblersRevealedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::GobblersRevealedFilter(decoded));
            }
            if let Ok(decoded) = GooBalanceUpdatedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::GooBalanceUpdatedFilter(decoded));
            }
            if let Ok(decoded) = LegendaryGobblerMintedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::LegendaryGobblerMintedFilter(decoded));
            }
            if let Ok(decoded) = RandProviderUpgradedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::RandProviderUpgradedFilter(decoded));
            }
            if let Ok(decoded) = RandomnessFulfilledFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::RandomnessFulfilledFilter(decoded));
            }
            if let Ok(decoded) = RandomnessRequestedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::RandomnessRequestedFilter(decoded));
            }
            if let Ok(decoded) = ReservedGobblersMintedFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::ReservedGobblersMintedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ArtGobblersEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ArtGobblersEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ArtGobblersEvents::ApprovalFilter(element) => element.fmt(f),
                ArtGobblersEvents::ApprovalForAllFilter(element) => element.fmt(f),
                ArtGobblersEvents::ArtGobbledFilter(element) => element.fmt(f),
                ArtGobblersEvents::GobblerClaimedFilter(element) => element.fmt(f),
                ArtGobblersEvents::GobblerPurchasedFilter(element) => element.fmt(f),
                ArtGobblersEvents::GobblersRevealedFilter(element) => element.fmt(f),
                ArtGobblersEvents::GooBalanceUpdatedFilter(element) => element.fmt(f),
                ArtGobblersEvents::LegendaryGobblerMintedFilter(element) => element.fmt(f),
                ArtGobblersEvents::RandProviderUpgradedFilter(element) => element.fmt(f),
                ArtGobblersEvents::RandomnessFulfilledFilter(element) => element.fmt(f),
                ArtGobblersEvents::RandomnessRequestedFilter(element) => element.fmt(f),
                ArtGobblersEvents::ReservedGobblersMintedFilter(element) => element.fmt(f),
                ArtGobblersEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
}
