pub use i_mystiko_bridge::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_mystiko_bridge {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct IMystikoBridge.DepositRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"hashK\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"randomS\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bridgeFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"executorFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rollupFee\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IMYSTIKOBRIDGE_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IMystikoBridge<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMystikoBridge<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMystikoBridge<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMystikoBridge<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMystikoBridge<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IMystikoBridge))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IMystikoBridge<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                IMYSTIKOBRIDGE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `deposit` (0x9a03636c) function
        pub fn deposit(
            &self,
            request: DepositRequest,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 3, 99, 108], (request,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for IMystikoBridge<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))` and selector `0x9a03636c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "deposit",
        abi = "deposit((uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256))"
    )]
    pub struct DepositCall {
        pub request: DepositRequest,
    }
    ///`DepositRequest(uint256,uint256,uint256,uint128,bytes,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositRequest {
        pub amount: ::ethers_core::types::U256,
        pub commitment: ::ethers_core::types::U256,
        pub hash_k: ::ethers_core::types::U256,
        pub random_s: u128,
        pub encrypted_note: ::ethers_core::types::Bytes,
        pub bridge_fee: ::ethers_core::types::U256,
        pub executor_fee: ::ethers_core::types::U256,
        pub rollup_fee: ::ethers_core::types::U256,
    }
}
