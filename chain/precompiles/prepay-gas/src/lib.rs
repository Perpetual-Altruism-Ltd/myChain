use fp_evm::{Context, ExitSucceed, PrecompileOutput, PrecompileFailure};
use pallet_evm::Precompile;
use gasometer::Gasometer;

pub struct TestPC1;

impl Precompile for TestPC1 {
	fn execute(
        _input: &[u8], 
        target_gas: Option<u64>, 
        _context: &Context,
		_is_static: bool,
    ) -> Result<PrecompileOutput, PrecompileFailure>{

		let mut gasometer = Gasometer::new(target_gas);
		let gasometer = &mut gasometer;

		let message: &str = "Hello, World from MyChain!";
		let result: Vec<u8> = message.as_bytes().to_vec();

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: result,
			logs: Default::default()
		})
	}
}
