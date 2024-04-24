pub use i_hasher_3::*;
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
pub mod i_hasher_3 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256[3]\",\"name\":\"data\",\"type\":\"uint256[3]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"poseidon\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IHASHER3_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IHasher3<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IHasher3<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IHasher3<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IHasher3<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IHasher3<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IHasher3))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IHasher3<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                IHASHER3_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `poseidon` (0x25cc70e8) function
        pub fn poseidon(
            &self,
            data: [::ethers_core::types::U256; 3],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::U256> {
            self.0
                .method_hash([37, 204, 112, 232], data)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>> for IHasher3<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `poseidon` function with signature `poseidon(uint256[3])` and selector `0x25cc70e8`
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
    #[ethcall(name = "poseidon", abi = "poseidon(uint256[3])")]
    pub struct PoseidonCall {
        pub data: [::ethers_core::types::U256; 3],
    }
    ///Container type for all return fields from the `poseidon` function with signature `poseidon(uint256[3])` and selector `0x25cc70e8`
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
    pub struct PoseidonReturn(pub ::ethers_core::types::U256);
}
