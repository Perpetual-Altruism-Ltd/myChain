
use evm_precompiles::{EvmDataWriter};
use utils::{Context, ExitError, ExitSucceed, Precompile, PrecompileOutput,
	 PrecompileFailure};

pub struct Blake2F;

impl Blake2F {
	const GAS_COST_PER_ROUND: u64 = 1; // https://eips.ethereum.org/EIPS/eip-152#gas-costs-and-benchmarks
}

pub struct TestPC1;

impl Precompile for TestPC1 {
	fn execute(
        input: &[u8], 
        target_gas: Option<u64>, 
        _context: &Context,
		_is_static: bool,
    ) -> Result<PrecompileOutput, PrecompileFailure>{

		let mut rounds_buf: [u8; 4] = [0; 4];
		rounds_buf.copy_from_slice(&input[0..4]);
		let rounds: u32 = u32::from_be_bytes(rounds_buf);

		let gas_cost: u64 = (rounds as u64) * Blake2F::GAS_COST_PER_ROUND;

		if let Some(gas_left) = target_gas {
			if gas_left < gas_cost {
				return Err(PrecompileFailure::Error{exit_status: ExitError::OutOfGas});
			}
		}

		let message: &str = "Hello, World from MyChain!";
		let result: Vec<u8> = message.as_bytes().to_vec();

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gas_cost,
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