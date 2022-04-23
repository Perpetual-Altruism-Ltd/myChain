
use evm_precompiles::{EvmDataWriter};
use fp_evm::{Context, ExitError, ExitSucceed, PrecompileOutput, PrecompileFailure};
use pallet_evm::Precompile;

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

		let gas_cost: u64 = rounds as u64;

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