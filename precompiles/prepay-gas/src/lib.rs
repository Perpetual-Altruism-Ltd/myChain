
use evm_precompiles::{Gasometer, EvmDataWriter};
use evm::{executor::stack::PrecompileOutput, Context, ExitSucceed, ExitError};


pub trait Precompile {
	fn execute(
		input: &[u8],
		target_gas: Option<u64>,
		context: &Context,
		gasometer: &mut Gasometer,
	) -> Result<PrecompileOutput, ExitError>;
}

pub struct TestPC1;

impl Precompile for TestPC1 {
	fn execute(
        _input: &[u8], 
        _target_gas: Option<u64>, 
        _context: &Context,
		gasometer: &mut Gasometer,
    ) -> Result<PrecompileOutput, ExitError>{

		let message: &str = "Hello, World from MyChain!";
		let result: Vec<u8> = message.as_bytes().to_vec();

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: EvmDataWriter::new().write(result).build(),
			logs: Default::default()
		})
	}
}


//leaving this here for referance
/*#![cfg_attr(not(feature = "std"), no_std)]

use pallet_evm::{ExitError, ExitSucceed, LinearCostPrecompile};
use sp_std::vec::Vec;

pub struct Deposit;

impl LinearCostPrecompile for Deposit
{
	const BASE: u64 = 0;
	const WORD: u64 = 0;

	fn execute(
		_input: &[u8],
		_cost: u64,
	) -> core::result::Result<(ExitSucceed, Vec<u8>), ExitError> {
		let message: &str = "Hello, World from MyChain!";
		let result: Vec<u8> = message.as_bytes().to_vec();
		Ok((ExitSucceed::Returned, result))
	}
}  */