pub use i_commitment_pool::*;
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
pub mod i_commitment_pool {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct ICommitmentPool.CommitmentRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"commitment\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"executorFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"rollupFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"encryptedNote\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"_executor\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"enqueue\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ICommitmentPool.RollupRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint32\",\"name\":\"rollupSize\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newRoot\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"leafHash\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rollup\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ICommitmentPool.TransactRequest\",\"name\":\"_request\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct IVerifier.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"rootHash\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"serialNumbers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"sigHashes\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"sigPk\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"publicAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"relayerFeeAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"outCommitments\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"outRollupFees\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"publicRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"relayerAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes[]\",\"name\":\"outEncryptedNotes\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"randomAuditingPublicKey\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"encryptedAuditorNotes\",\"type\":\"uint256[]\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"_signature\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transact\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICOMMITMENTPOOL_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ICommitmentPool<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICommitmentPool<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICommitmentPool<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICommitmentPool<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICommitmentPool<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ICommitmentPool))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> ICommitmentPool<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                ICOMMITMENTPOOL_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `enqueue` (0x78d60cd7) function
        pub fn enqueue(
            &self,
            request: CommitmentRequest,
            executor: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 214, 12, 215], (request, executor))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollup` (0x14a7737d) function
        pub fn rollup(
            &self,
            request: RollupRequest,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 167, 115, 125], (request,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transact` (0x72082971) function
        pub fn transact(
            &self,
            request: TransactRequest,
            signature: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 8, 41, 113], (request, signature))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
        for ICommitmentPool<M>
    {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `enqueue` function with signature `enqueue((uint256,uint256,uint256,uint256,bytes),address)` and selector `0x78d60cd7`
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
        name = "enqueue",
        abi = "enqueue((uint256,uint256,uint256,uint256,bytes),address)"
    )]
    pub struct EnqueueCall {
        pub request: CommitmentRequest,
        pub executor: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `rollup` function with signature `rollup((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256))` and selector `0x14a7737d`
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
        name = "rollup",
        abi = "rollup((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256))"
    )]
    pub struct RollupCall {
        pub request: RollupRequest,
    }
    ///Container type for all input parameters for the `transact` function with signature `transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)` and selector `0x72082971`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[ethcall(
        name = "transact",
        abi = "transact((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[]),bytes)"
    )]
    pub struct TransactCall {
        pub request: TransactRequest,
        pub signature: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, serde::Serialize, serde::Deserialize)]
    pub enum ICommitmentPoolCalls {
        Enqueue(EnqueueCall),
        Rollup(RollupCall),
        Transact(TransactCall),
    }
    impl ::ethers_core::abi::AbiDecode for ICommitmentPoolCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EnqueueCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Enqueue(decoded));
            }
            if let Ok(decoded) = <RollupCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rollup(decoded));
            }
            if let Ok(decoded) = <TransactCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transact(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for ICommitmentPoolCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Enqueue(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Rollup(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Transact(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ICommitmentPoolCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Enqueue(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rollup(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transact(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EnqueueCall> for ICommitmentPoolCalls {
        fn from(value: EnqueueCall) -> Self {
            Self::Enqueue(value)
        }
    }
    impl ::core::convert::From<RollupCall> for ICommitmentPoolCalls {
        fn from(value: RollupCall) -> Self {
            Self::Rollup(value)
        }
    }
    impl ::core::convert::From<TransactCall> for ICommitmentPoolCalls {
        fn from(value: TransactCall) -> Self {
            Self::Transact(value)
        }
    }
    ///`CommitmentRequest(uint256,uint256,uint256,uint256,bytes)`
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
    pub struct CommitmentRequest {
        pub amount: ::ethers_core::types::U256,
        pub commitment: ::ethers_core::types::U256,
        pub executor_fee: ::ethers_core::types::U256,
        pub rollup_fee: ::ethers_core::types::U256,
        pub encrypted_note: ::ethers_core::types::Bytes,
    }
    ///`RollupRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint32,uint256,uint256)`
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
    pub struct RollupRequest {
        pub proof: Proof,
        pub rollup_size: u32,
        pub new_root: ::ethers_core::types::U256,
        pub leaf_hash: ::ethers_core::types::U256,
    }
    ///`TransactRequest(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256,uint256[],uint256[],bytes32,uint256,uint256,uint256[],uint256[],address,address,bytes[],uint256,uint256[])`
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
    pub struct TransactRequest {
        pub proof: Proof,
        pub root_hash: ::ethers_core::types::U256,
        pub serial_numbers: ::std::vec::Vec<::ethers_core::types::U256>,
        pub sig_hashes: ::std::vec::Vec<::ethers_core::types::U256>,
        pub sig_pk: [u8; 32],
        pub public_amount: ::ethers_core::types::U256,
        pub relayer_fee_amount: ::ethers_core::types::U256,
        pub out_commitments: ::std::vec::Vec<::ethers_core::types::U256>,
        pub out_rollup_fees: ::std::vec::Vec<::ethers_core::types::U256>,
        pub public_recipient: ::ethers_core::types::Address,
        pub relayer_address: ::ethers_core::types::Address,
        pub out_encrypted_notes: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub random_auditing_public_key: ::ethers_core::types::U256,
        pub encrypted_auditor_notes: ::std::vec::Vec<::ethers_core::types::U256>,
    }
    ///`G1Point(uint256,uint256)`
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
    pub struct G1Point {
        pub x: ::ethers_core::types::U256,
        pub y: ::ethers_core::types::U256,
    }
    ///`G2Point(uint256[2],uint256[2])`
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
    pub struct G2Point {
        pub x: [::ethers_core::types::U256; 2],
        pub y: [::ethers_core::types::U256; 2],
    }
    ///`Proof((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`
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
    pub struct Proof {
        pub a: G1Point,
        pub b: G2Point,
        pub c: G1Point,
    }
}
