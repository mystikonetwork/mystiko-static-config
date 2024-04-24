pub use mystiko_gas_relayer_register::*;
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
pub mod mystiko_gas_relayer_register {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"url\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasRelayerDeRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasRelayerNameUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"url\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasRelayerRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"url\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasRelayerUrlUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"worker\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasRelayerWorkerDeRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"worker\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"GasRelayerWorkerRegistered\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OperatorChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newOperator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeOperator\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deregisterRelayer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_worker\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deregisterWorker\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllRelayerInfo\",\"outputs\":[{\"internalType\":\"string[]\",\"name\":\"\",\"type\":\"string[]\",\"components\":[]},{\"internalType\":\"string[]\",\"name\":\"\",\"type\":\"string[]\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_worker\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRelayerUrlAndName\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_worker\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isWorkerRegistered\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"operator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_relayerUrl\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_relayerName\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"_relayerWorkers\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerRelayer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_relayer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_newWorker\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"registerWorker\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayerMetaMap\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"url\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayerNameMap\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayerUrlMap\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"relayers\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"relayer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"removeRelayer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_relayer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_relayerName\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateRelayerName\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_relayer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_relayerUrl\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateRelayerUrl\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"workerMap\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static MYSTIKOGASRELAYERREGISTER_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct MystikoGasRelayerRegister<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MystikoGasRelayerRegister<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MystikoGasRelayerRegister<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MystikoGasRelayerRegister<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MystikoGasRelayerRegister<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(MystikoGasRelayerRegister))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> MystikoGasRelayerRegister<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MYSTIKOGASRELAYERREGISTER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `changeOperator` (0x06394c9b) function
        pub fn change_operator(
            &self,
            new_operator: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([6, 57, 76, 155], new_operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterRelayer` (0x57f914eb) function
        pub fn deregister_relayer(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 249, 20, 235], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deregisterWorker` (0xd628d7b2) function
        pub fn deregister_worker(
            &self,
            worker: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 40, 215, 178], worker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllRelayerInfo` (0x045192ee) function
        pub fn get_all_relayer_info(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::std::string::String>,
                ::std::vec::Vec<::std::string::String>,
                ::std::vec::Vec<::ethers_core::types::Address>,
            ),
        > {
            self.0
                .method_hash([4, 81, 146, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRelayerUrlAndName` (0xdd945e4f) function
        pub fn get_relayer_url_and_name(
            &self,
            worker: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (::std::string::String, ::std::string::String),
        > {
            self.0
                .method_hash([221, 148, 94, 79], worker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isWorkerRegistered` (0xe798a7da) function
        pub fn is_worker_registered(
            &self,
            worker: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([231, 152, 167, 218], worker)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operator` (0x570ca735) function
        pub fn operator(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([87, 12, 167, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerRelayer` (0xde9bfb8d) function
        pub fn register_relayer(
            &self,
            relayer_url: ::std::string::String,
            relayer_name: ::std::string::String,
            relayer_workers: ::std::vec::Vec<::ethers_core::types::Address>,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [222, 155, 251, 141],
                    (relayer_url, relayer_name, relayer_workers),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerWorker` (0x2e650649) function
        pub fn register_worker(
            &self,
            relayer: ::ethers_core::types::Address,
            new_worker: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 101, 6, 73], (relayer, new_worker))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerMetaMap` (0xd7b8f526) function
        pub fn relayer_meta_map(
            &self,
            p0: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (::std::string::String, ::std::string::String),
        > {
            self.0
                .method_hash([215, 184, 245, 38], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerNameMap` (0x4c45235e) function
        pub fn relayer_name_map(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([76, 69, 35, 94], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayerUrlMap` (0x194e578f) function
        pub fn relayer_url_map(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([25, 78, 87, 143], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `relayers` (0x9a48e7f9) function
        pub fn relayers(
            &self,
            p0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([154, 72, 231, 249], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeRelayer` (0x60f0a5ac) function
        pub fn remove_relayer(
            &self,
            relayer: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 240, 165, 172], relayer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateRelayerName` (0x89bd0726) function
        pub fn update_relayer_name(
            &self,
            relayer: ::ethers_core::types::Address,
            relayer_name: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 189, 7, 38], (relayer, relayer_name))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateRelayerUrl` (0xcf1467fe) function
        pub fn update_relayer_url(
            &self,
            relayer: ::ethers_core::types::Address,
            relayer_url: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 20, 103, 254], (relayer, relayer_url))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `workerMap` (0x39f450d7) function
        pub fn worker_map(
            &self,
            p0: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers_core::types::Address> {
            self.0
                .method_hash([57, 244, 80, 215], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `GasRelayerDeRegistered` event
        pub fn gas_relayer_de_registered_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, GasRelayerDeRegisteredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `GasRelayerNameUpdate` event
        pub fn gas_relayer_name_update_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, GasRelayerNameUpdateFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `GasRelayerRegistered` event
        pub fn gas_relayer_registered_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, GasRelayerRegisteredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `GasRelayerUrlUpdate` event
        pub fn gas_relayer_url_update_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, GasRelayerUrlUpdateFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `GasRelayerWorkerDeRegistered` event
        pub fn gas_relayer_worker_de_registered_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasRelayerWorkerDeRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GasRelayerWorkerRegistered` event
        pub fn gas_relayer_worker_registered_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GasRelayerWorkerRegisteredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OperatorChanged` event
        pub fn operator_changed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, OperatorChangedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MystikoGasRelayerRegisterEvents,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
        for MystikoGasRelayerRegister<M>
    {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasRelayerDeRegistered",
        abi = "GasRelayerDeRegistered(address,string,string)"
    )]
    pub struct GasRelayerDeRegisteredFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers_core::types::Address,
        pub url: ::std::string::String,
        pub name: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasRelayerNameUpdate",
        abi = "GasRelayerNameUpdate(address,string)"
    )]
    pub struct GasRelayerNameUpdateFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers_core::types::Address,
        pub name: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasRelayerRegistered",
        abi = "GasRelayerRegistered(address,string,string)"
    )]
    pub struct GasRelayerRegisteredFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers_core::types::Address,
        pub url: ::std::string::String,
        pub name: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasRelayerUrlUpdate",
        abi = "GasRelayerUrlUpdate(address,string)"
    )]
    pub struct GasRelayerUrlUpdateFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers_core::types::Address,
        pub url: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasRelayerWorkerDeRegistered",
        abi = "GasRelayerWorkerDeRegistered(address,address)"
    )]
    pub struct GasRelayerWorkerDeRegisteredFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers_core::types::Address,
        pub worker: ::ethers_core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GasRelayerWorkerRegistered",
        abi = "GasRelayerWorkerRegistered(address,address)"
    )]
    pub struct GasRelayerWorkerRegisteredFilter {
        #[ethevent(indexed)]
        pub relayer: ::ethers_core::types::Address,
        pub worker: ::ethers_core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OperatorChanged", abi = "OperatorChanged(address)")]
    pub struct OperatorChangedFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers_core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum MystikoGasRelayerRegisterEvents {
        GasRelayerDeRegisteredFilter(GasRelayerDeRegisteredFilter),
        GasRelayerNameUpdateFilter(GasRelayerNameUpdateFilter),
        GasRelayerRegisteredFilter(GasRelayerRegisteredFilter),
        GasRelayerUrlUpdateFilter(GasRelayerUrlUpdateFilter),
        GasRelayerWorkerDeRegisteredFilter(GasRelayerWorkerDeRegisteredFilter),
        GasRelayerWorkerRegisteredFilter(GasRelayerWorkerRegisteredFilter),
        OperatorChangedFilter(OperatorChangedFilter),
    }
    impl ::ethers_contract::EthLogDecode for MystikoGasRelayerRegisterEvents {
        fn decode_log(
            log: &::ethers_core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::Error> {
            if let Ok(decoded) = GasRelayerDeRegisteredFilter::decode_log(log) {
                return Ok(MystikoGasRelayerRegisterEvents::GasRelayerDeRegisteredFilter(decoded));
            }
            if let Ok(decoded) = GasRelayerNameUpdateFilter::decode_log(log) {
                return Ok(MystikoGasRelayerRegisterEvents::GasRelayerNameUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = GasRelayerRegisteredFilter::decode_log(log) {
                return Ok(MystikoGasRelayerRegisterEvents::GasRelayerRegisteredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = GasRelayerUrlUpdateFilter::decode_log(log) {
                return Ok(MystikoGasRelayerRegisterEvents::GasRelayerUrlUpdateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = GasRelayerWorkerDeRegisteredFilter::decode_log(log) {
                return Ok(
                    MystikoGasRelayerRegisterEvents::GasRelayerWorkerDeRegisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = GasRelayerWorkerRegisteredFilter::decode_log(log) {
                return Ok(
                    MystikoGasRelayerRegisterEvents::GasRelayerWorkerRegisteredFilter(decoded),
                );
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(MystikoGasRelayerRegisterEvents::OperatorChangedFilter(
                    decoded,
                ));
            }
            Err(::ethers_core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MystikoGasRelayerRegisterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GasRelayerDeRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GasRelayerNameUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasRelayerRegisteredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasRelayerUrlUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GasRelayerWorkerDeRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GasRelayerWorkerRegisteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GasRelayerDeRegisteredFilter> for MystikoGasRelayerRegisterEvents {
        fn from(value: GasRelayerDeRegisteredFilter) -> Self {
            Self::GasRelayerDeRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<GasRelayerNameUpdateFilter> for MystikoGasRelayerRegisterEvents {
        fn from(value: GasRelayerNameUpdateFilter) -> Self {
            Self::GasRelayerNameUpdateFilter(value)
        }
    }
    impl ::core::convert::From<GasRelayerRegisteredFilter> for MystikoGasRelayerRegisterEvents {
        fn from(value: GasRelayerRegisteredFilter) -> Self {
            Self::GasRelayerRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<GasRelayerUrlUpdateFilter> for MystikoGasRelayerRegisterEvents {
        fn from(value: GasRelayerUrlUpdateFilter) -> Self {
            Self::GasRelayerUrlUpdateFilter(value)
        }
    }
    impl ::core::convert::From<GasRelayerWorkerDeRegisteredFilter> for MystikoGasRelayerRegisterEvents {
        fn from(value: GasRelayerWorkerDeRegisteredFilter) -> Self {
            Self::GasRelayerWorkerDeRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<GasRelayerWorkerRegisteredFilter> for MystikoGasRelayerRegisterEvents {
        fn from(value: GasRelayerWorkerRegisteredFilter) -> Self {
            Self::GasRelayerWorkerRegisteredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for MystikoGasRelayerRegisterEvents {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `changeOperator` function with signature `changeOperator(address)` and selector `0x06394c9b`
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
    #[ethcall(name = "changeOperator", abi = "changeOperator(address)")]
    pub struct ChangeOperatorCall {
        pub new_operator: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `deregisterRelayer` function with signature `deregisterRelayer()` and selector `0x57f914eb`
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
    #[ethcall(name = "deregisterRelayer", abi = "deregisterRelayer()")]
    pub struct DeregisterRelayerCall;
    ///Container type for all input parameters for the `deregisterWorker` function with signature `deregisterWorker(address)` and selector `0xd628d7b2`
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
    #[ethcall(name = "deregisterWorker", abi = "deregisterWorker(address)")]
    pub struct DeregisterWorkerCall {
        pub worker: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `getAllRelayerInfo` function with signature `getAllRelayerInfo()` and selector `0x045192ee`
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
    #[ethcall(name = "getAllRelayerInfo", abi = "getAllRelayerInfo()")]
    pub struct GetAllRelayerInfoCall;
    ///Container type for all input parameters for the `getRelayerUrlAndName` function with signature `getRelayerUrlAndName(address)` and selector `0xdd945e4f`
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
    #[ethcall(name = "getRelayerUrlAndName", abi = "getRelayerUrlAndName(address)")]
    pub struct GetRelayerUrlAndNameCall {
        pub worker: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `isWorkerRegistered` function with signature `isWorkerRegistered(address)` and selector `0xe798a7da`
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
    #[ethcall(name = "isWorkerRegistered", abi = "isWorkerRegistered(address)")]
    pub struct IsWorkerRegisteredCall {
        pub worker: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `operator` function with signature `operator()` and selector `0x570ca735`
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
    #[ethcall(name = "operator", abi = "operator()")]
    pub struct OperatorCall;
    ///Container type for all input parameters for the `registerRelayer` function with signature `registerRelayer(string,string,address[])` and selector `0xde9bfb8d`
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
        name = "registerRelayer",
        abi = "registerRelayer(string,string,address[])"
    )]
    pub struct RegisterRelayerCall {
        pub relayer_url: ::std::string::String,
        pub relayer_name: ::std::string::String,
        pub relayer_workers: ::std::vec::Vec<::ethers_core::types::Address>,
    }
    ///Container type for all input parameters for the `registerWorker` function with signature `registerWorker(address,address)` and selector `0x2e650649`
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
    #[ethcall(name = "registerWorker", abi = "registerWorker(address,address)")]
    pub struct RegisterWorkerCall {
        pub relayer: ::ethers_core::types::Address,
        pub new_worker: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `relayerMetaMap` function with signature `relayerMetaMap(address)` and selector `0xd7b8f526`
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
    #[ethcall(name = "relayerMetaMap", abi = "relayerMetaMap(address)")]
    pub struct RelayerMetaMapCall(pub ::ethers_core::types::Address);
    ///Container type for all input parameters for the `relayerNameMap` function with signature `relayerNameMap(string)` and selector `0x4c45235e`
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
    #[ethcall(name = "relayerNameMap", abi = "relayerNameMap(string)")]
    pub struct RelayerNameMapCall(pub ::std::string::String);
    ///Container type for all input parameters for the `relayerUrlMap` function with signature `relayerUrlMap(string)` and selector `0x194e578f`
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
    #[ethcall(name = "relayerUrlMap", abi = "relayerUrlMap(string)")]
    pub struct RelayerUrlMapCall(pub ::std::string::String);
    ///Container type for all input parameters for the `relayers` function with signature `relayers(uint256)` and selector `0x9a48e7f9`
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
    #[ethcall(name = "relayers", abi = "relayers(uint256)")]
    pub struct RelayersCall(pub ::ethers_core::types::U256);
    ///Container type for all input parameters for the `removeRelayer` function with signature `removeRelayer(address)` and selector `0x60f0a5ac`
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
    #[ethcall(name = "removeRelayer", abi = "removeRelayer(address)")]
    pub struct RemoveRelayerCall {
        pub relayer: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `updateRelayerName` function with signature `updateRelayerName(address,string)` and selector `0x89bd0726`
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
    #[ethcall(name = "updateRelayerName", abi = "updateRelayerName(address,string)")]
    pub struct UpdateRelayerNameCall {
        pub relayer: ::ethers_core::types::Address,
        pub relayer_name: ::std::string::String,
    }
    ///Container type for all input parameters for the `updateRelayerUrl` function with signature `updateRelayerUrl(address,string)` and selector `0xcf1467fe`
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
    #[ethcall(name = "updateRelayerUrl", abi = "updateRelayerUrl(address,string)")]
    pub struct UpdateRelayerUrlCall {
        pub relayer: ::ethers_core::types::Address,
        pub relayer_url: ::std::string::String,
    }
    ///Container type for all input parameters for the `workerMap` function with signature `workerMap(address)` and selector `0x39f450d7`
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
    #[ethcall(name = "workerMap", abi = "workerMap(address)")]
    pub struct WorkerMapCall(pub ::ethers_core::types::Address);
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum MystikoGasRelayerRegisterCalls {
        ChangeOperator(ChangeOperatorCall),
        DeregisterRelayer(DeregisterRelayerCall),
        DeregisterWorker(DeregisterWorkerCall),
        GetAllRelayerInfo(GetAllRelayerInfoCall),
        GetRelayerUrlAndName(GetRelayerUrlAndNameCall),
        IsWorkerRegistered(IsWorkerRegisteredCall),
        Operator(OperatorCall),
        RegisterRelayer(RegisterRelayerCall),
        RegisterWorker(RegisterWorkerCall),
        RelayerMetaMap(RelayerMetaMapCall),
        RelayerNameMap(RelayerNameMapCall),
        RelayerUrlMap(RelayerUrlMapCall),
        Relayers(RelayersCall),
        RemoveRelayer(RemoveRelayerCall),
        UpdateRelayerName(UpdateRelayerNameCall),
        UpdateRelayerUrl(UpdateRelayerUrlCall),
        WorkerMap(WorkerMapCall),
    }
    impl ::ethers_core::abi::AbiDecode for MystikoGasRelayerRegisterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ChangeOperatorCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChangeOperator(decoded));
            }
            if let Ok(decoded) =
                <DeregisterRelayerCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterRelayer(decoded));
            }
            if let Ok(decoded) =
                <DeregisterWorkerCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeregisterWorker(decoded));
            }
            if let Ok(decoded) =
                <GetAllRelayerInfoCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllRelayerInfo(decoded));
            }
            if let Ok(decoded) =
                <GetRelayerUrlAndNameCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetRelayerUrlAndName(decoded));
            }
            if let Ok(decoded) =
                <IsWorkerRegisteredCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsWorkerRegistered(decoded));
            }
            if let Ok(decoded) = <OperatorCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Operator(decoded));
            }
            if let Ok(decoded) =
                <RegisterRelayerCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterRelayer(decoded));
            }
            if let Ok(decoded) = <RegisterWorkerCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterWorker(decoded));
            }
            if let Ok(decoded) = <RelayerMetaMapCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RelayerMetaMap(decoded));
            }
            if let Ok(decoded) = <RelayerNameMapCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RelayerNameMap(decoded));
            }
            if let Ok(decoded) = <RelayerUrlMapCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RelayerUrlMap(decoded));
            }
            if let Ok(decoded) = <RelayersCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Relayers(decoded));
            }
            if let Ok(decoded) = <RemoveRelayerCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveRelayer(decoded));
            }
            if let Ok(decoded) =
                <UpdateRelayerNameCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateRelayerName(decoded));
            }
            if let Ok(decoded) =
                <UpdateRelayerUrlCall as ::ethers_core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateRelayerUrl(decoded));
            }
            if let Ok(decoded) = <WorkerMapCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WorkerMap(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for MystikoGasRelayerRegisterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ChangeOperator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DeregisterRelayer(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::DeregisterWorker(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetAllRelayerInfo(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::GetRelayerUrlAndName(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::IsWorkerRegistered(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Operator(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RegisterRelayer(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RegisterWorker(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RelayerMetaMap(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RelayerNameMap(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RelayerUrlMap(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Relayers(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::RemoveRelayer(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateRelayerName(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UpdateRelayerUrl(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::WorkerMap(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MystikoGasRelayerRegisterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChangeOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterRelayer(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeregisterWorker(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllRelayerInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRelayerUrlAndName(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsWorkerRegistered(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operator(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterRelayer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterWorker(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerMetaMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerNameMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayerUrlMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Relayers(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveRelayer(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRelayerName(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateRelayerUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::WorkerMap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChangeOperatorCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: ChangeOperatorCall) -> Self {
            Self::ChangeOperator(value)
        }
    }
    impl ::core::convert::From<DeregisterRelayerCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: DeregisterRelayerCall) -> Self {
            Self::DeregisterRelayer(value)
        }
    }
    impl ::core::convert::From<DeregisterWorkerCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: DeregisterWorkerCall) -> Self {
            Self::DeregisterWorker(value)
        }
    }
    impl ::core::convert::From<GetAllRelayerInfoCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: GetAllRelayerInfoCall) -> Self {
            Self::GetAllRelayerInfo(value)
        }
    }
    impl ::core::convert::From<GetRelayerUrlAndNameCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: GetRelayerUrlAndNameCall) -> Self {
            Self::GetRelayerUrlAndName(value)
        }
    }
    impl ::core::convert::From<IsWorkerRegisteredCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: IsWorkerRegisteredCall) -> Self {
            Self::IsWorkerRegistered(value)
        }
    }
    impl ::core::convert::From<OperatorCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: OperatorCall) -> Self {
            Self::Operator(value)
        }
    }
    impl ::core::convert::From<RegisterRelayerCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: RegisterRelayerCall) -> Self {
            Self::RegisterRelayer(value)
        }
    }
    impl ::core::convert::From<RegisterWorkerCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: RegisterWorkerCall) -> Self {
            Self::RegisterWorker(value)
        }
    }
    impl ::core::convert::From<RelayerMetaMapCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: RelayerMetaMapCall) -> Self {
            Self::RelayerMetaMap(value)
        }
    }
    impl ::core::convert::From<RelayerNameMapCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: RelayerNameMapCall) -> Self {
            Self::RelayerNameMap(value)
        }
    }
    impl ::core::convert::From<RelayerUrlMapCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: RelayerUrlMapCall) -> Self {
            Self::RelayerUrlMap(value)
        }
    }
    impl ::core::convert::From<RelayersCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: RelayersCall) -> Self {
            Self::Relayers(value)
        }
    }
    impl ::core::convert::From<RemoveRelayerCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: RemoveRelayerCall) -> Self {
            Self::RemoveRelayer(value)
        }
    }
    impl ::core::convert::From<UpdateRelayerNameCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: UpdateRelayerNameCall) -> Self {
            Self::UpdateRelayerName(value)
        }
    }
    impl ::core::convert::From<UpdateRelayerUrlCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: UpdateRelayerUrlCall) -> Self {
            Self::UpdateRelayerUrl(value)
        }
    }
    impl ::core::convert::From<WorkerMapCall> for MystikoGasRelayerRegisterCalls {
        fn from(value: WorkerMapCall) -> Self {
            Self::WorkerMap(value)
        }
    }
    ///Container type for all return fields from the `getAllRelayerInfo` function with signature `getAllRelayerInfo()` and selector `0x045192ee`
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
    pub struct GetAllRelayerInfoReturn(
        pub ::std::vec::Vec<::std::string::String>,
        pub ::std::vec::Vec<::std::string::String>,
        pub ::std::vec::Vec<::ethers_core::types::Address>,
    );
    ///Container type for all return fields from the `getRelayerUrlAndName` function with signature `getRelayerUrlAndName(address)` and selector `0xdd945e4f`
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
    pub struct GetRelayerUrlAndNameReturn(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all return fields from the `isWorkerRegistered` function with signature `isWorkerRegistered(address)` and selector `0xe798a7da`
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
    pub struct IsWorkerRegisteredReturn(pub bool);
    ///Container type for all return fields from the `operator` function with signature `operator()` and selector `0x570ca735`
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
    pub struct OperatorReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `relayerMetaMap` function with signature `relayerMetaMap(address)` and selector `0xd7b8f526`
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
    pub struct RelayerMetaMapReturn {
        pub url: ::std::string::String,
        pub name: ::std::string::String,
    }
    ///Container type for all return fields from the `relayerNameMap` function with signature `relayerNameMap(string)` and selector `0x4c45235e`
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
    pub struct RelayerNameMapReturn(pub bool);
    ///Container type for all return fields from the `relayerUrlMap` function with signature `relayerUrlMap(string)` and selector `0x194e578f`
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
    pub struct RelayerUrlMapReturn(pub bool);
    ///Container type for all return fields from the `relayers` function with signature `relayers(uint256)` and selector `0x9a48e7f9`
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
    pub struct RelayersReturn(pub ::ethers_core::types::Address);
    ///Container type for all return fields from the `workerMap` function with signature `workerMap(address)` and selector `0x39f450d7`
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
    pub struct WorkerMapReturn(pub ::ethers_core::types::Address);
}
