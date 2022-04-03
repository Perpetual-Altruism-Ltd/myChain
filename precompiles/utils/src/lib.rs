use primitive_types::{H160, H256, U256};
use pallet_evm::{ExitError, ExitSucceed, Log, ExitRevert};
use evm::PrecompileFailure;

/// Create scheme.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CreateScheme {
	/// Legacy create scheme of `CREATE`.
	Legacy {
		/// Caller of the create.
		caller: H160,
	},
	/// Create scheme of `CREATE2`.
	Create2 {
		/// Caller of the create.
		caller: H160,
		/// Code hash.
		code_hash: H256,
		/// Salt.
		salt: H256,
	},
	/// Create at a fixed location.
	Fixed(H160),
}

/// Call scheme.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CallScheme {
	/// `CALL`
	Call,
	/// `CALLCODE`
	CallCode,
	/// `DELEGATECALL`
	DelegateCall,
	/// `STATICCALL`
	StaticCall,
}

/// Context of the runtime.
#[derive(Clone, Debug)]
pub struct Context {
	/// Execution address.
	pub address: H160,
	/// Caller of the EVM.
	pub caller: H160,
	/// Apparent value of the EVM.
	pub apparent_value: U256,
}

pub struct PrecompileOutput {
    pub exit_status: ExitSucceed,
    pub cost: u64,
    pub output: Vec<u8>,
    pub logs: Vec<Log>,
}





pub type EvmResult<T = ()> = Result<T, PrecompileFailure>;

/// Represents modifiers a Solidity function can be annotated with.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FunctionModifier {
	/// Function that doesn't modify the state.
	View,
	/// Function that modifies the state but refuse receiving funds.
	/// Correspond to a Solidity function with no modifiers.
	NonPayable,
	/// Function that modifies the state and accept funds.
	Payable,
}

/// Custom Gasometer to record costs in precompiles.
/// It is advised to record known costs as early as possible to
/// avoid unecessary computations if there is an Out of Gas.
///
/// Provides functions related to reverts, as reverts takes the recorded amount
/// of gas into account.
#[derive(Clone, Copy, Debug)]
pub struct Gasometer {
	target_gas: Option<u64>,
	used_gas: u64,
}

impl Gasometer {
	/// Create a new Gasometer with provided gas limit.
	/// None is no limit.
	pub fn new(target_gas: Option<u64>) -> Self {
		Self {
			target_gas,
			used_gas: 0,
		}
	}

	/// Get used gas.
	pub fn used_gas(&self) -> u64 {
		self.used_gas
	}

	/// Record cost, and return error if it goes out of gas.
	#[must_use]
	pub fn record_cost(&mut self, cost: u64) -> EvmResult {
		self.used_gas = self
			.used_gas
			.checked_add(cost)
			.ok_or(PrecompileFailure::Error {
				exit_status: ExitError::OutOfGas,
			})?;

		match self.target_gas {
			Some(gas_limit) if self.used_gas > gas_limit => Err(PrecompileFailure::Error {
				exit_status: ExitError::OutOfGas,
			}),
			_ => Ok(()),
		}
	}

	/// Record cost of a log manualy.
	/// This can be useful to record log costs early when their content have static size.
	#[must_use]
	pub fn record_log_costs_manual(&mut self, topics: usize, data_len: usize) -> EvmResult {
		// Cost calculation is copied from EVM code that is not publicly exposed by the crates.
		// https://github.com/rust-blockchain/evm/blob/master/gasometer/src/costs.rs#L148

		const G_LOG: u64 = 375;
		const G_LOGDATA: u64 = 8;
		const G_LOGTOPIC: u64 = 375;

		let topic_cost = G_LOGTOPIC
			.checked_mul(topics as u64)
			.ok_or(PrecompileFailure::Error {
				exit_status: ExitError::OutOfGas,
			})?;

		let data_cost = G_LOGDATA
			.checked_mul(data_len as u64)
			.ok_or(PrecompileFailure::Error {
				exit_status: ExitError::OutOfGas,
			})?;

		self.record_cost(G_LOG)?;
		self.record_cost(topic_cost)?;
		self.record_cost(data_cost)?;

		Ok(())
	}

	/// Record cost of logs.
	#[must_use]
	pub fn record_log_costs(&mut self, logs: &[Log]) -> EvmResult {
		for log in logs {
			self.record_log_costs_manual(log.topics.len(), log.data.len())?;
		}

		Ok(())
	}

	/// Compute remaining gas.
	/// Returns error if out of gas.
	/// Returns None if no gas limit.
	#[must_use]
	pub fn remaining_gas(&self) -> EvmResult<Option<u64>> {
		Ok(match self.target_gas {
			None => None,
			Some(gas_limit) => Some(gas_limit.checked_sub(self.used_gas).ok_or(
				PrecompileFailure::Error {
					exit_status: ExitError::OutOfGas,
				},
			)?),
		})
	}

	/// Revert the execution, making the user pay for the the currently
	/// recorded cost. It is better to **revert** instead of **error** as
	/// erroring consumes the entire gas limit, and **revert** returns an error
	/// message to the calling contract.
	///
	/// TODO : Record cost of the input based on its size and handle Out of Gas ?
	/// This might be required if we format revert messages using user data.
	#[must_use]
	pub fn revert(&self, output: impl AsRef<[u8]>) -> PrecompileFailure {
		PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: output.as_ref().to_owned(),
			cost: self.used_gas,
		}
	}

	#[must_use]
	/// Check that a function call is compatible with the context it is
	/// called into.
	pub fn check_function_modifier(
		&self,
		context: &Context,
		is_static: bool,
		modifier: FunctionModifier,
	) -> EvmResult {
		if is_static && modifier != FunctionModifier::View {
			return Err(self.revert("can't call non-static function in static context"));
		}

		if modifier != FunctionModifier::Payable && context.apparent_value > U256::zero() {
			return Err(self.revert("function is not payable"));
		}

		Ok(())
	}
}