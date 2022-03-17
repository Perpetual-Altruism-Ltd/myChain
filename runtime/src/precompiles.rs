use pallet_evm::{ExitError, ExitSucceed, LinearCostPrecompile};
/// Leaving the rest of the imports here for reference. 
/// use pallet_evm::{PrecompileSet, Precompile}; 
/// Compare to https://docs.rs/pallet-evm/4.0.0/pallet_evm/trait.Precompile.html which also calculates the amount of gas necessary.

/// Keeping H160 for later use. 
/// use sp_core::H160;
use sp_std::vec::Vec;

/// Basic example of a precompile struct - the execute function will be called at the address respective to the list order of the precompiles
/// assigned to the Precompiles type on the pallet_evm::Config implementation. Doing a static call to the address of this contract will return
/// the hello world message in base16. 
pub struct SampleMyChainPrecompile;

impl LinearCostPrecompile for SampleMyChainPrecompile
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
}