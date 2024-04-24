use crate::raw::gas_cost::RawGasCostConfig;
use crate::raw::transaction_info::RawTransactionInfoConfig;
use anyhow::Result;
use std::sync::Arc;
use validator::Validate;

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionInfoConfig {
    main_gas_cost: MainGasCost,
    erc20_gas_cost: Erc20GasCost,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MainGasCost {
    raw: Arc<RawGasCostConfig>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Erc20GasCost {
    raw: Arc<RawGasCostConfig>,
}

impl TransactionInfoConfig {
    pub fn new(raw: Arc<RawTransactionInfoConfig>) -> Self {
        let main_gas_cost = MainGasCost::new(raw.main_gas_cost.clone());
        let erc20_gas_cost = Erc20GasCost::new(raw.erc20_gas_cost.clone());
        TransactionInfoConfig {
            main_gas_cost,
            erc20_gas_cost,
        }
    }

    pub fn main_gas_cost(&self) -> &MainGasCost {
        &self.main_gas_cost
    }

    pub fn erc20_gas_cost(&self) -> &Erc20GasCost {
        &self.erc20_gas_cost
    }

    pub fn validate(&self) -> Result<()> {
        self.main_gas_cost.validate()?;
        self.erc20_gas_cost.validate()
    }
}

impl MainGasCost {
    pub fn new(raw: Arc<RawGasCostConfig>) -> Self {
        MainGasCost { raw }
    }

    pub fn gas_cost_config(&self) -> &RawGasCostConfig {
        self.raw.as_ref()
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}

impl Erc20GasCost {
    pub fn new(raw: Arc<RawGasCostConfig>) -> Self {
        Erc20GasCost { raw }
    }

    pub fn gas_cost_config(&self) -> &RawGasCostConfig {
        self.raw.as_ref()
    }

    pub fn validate(&self) -> Result<()> {
        Ok(self.raw.validate()?)
    }
}
