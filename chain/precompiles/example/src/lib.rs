
use frame_support::{traits::StorageInstance, weights::{GetDispatchInfo, PostDispatchInfo}, dispatch::Dispatchable};
use fp_evm::{Context, ExitSucceed, PrecompileOutput, PrecompileFailure, Precompile};
use gasometer::{Gasometer, FunctionModifier, RuntimeHelper, EvmResult, LogsBuilder};
use evmdata::{EvmDataWriter, EvmDataReader};
use sp_core::U256;
use sp_std::marker::PhantomData;
use FunctionSelector::keccak256;


#[FunctionSelector::generate_function_selector]
enum Action {
	Deposit = "deposit(address,uint256)",
	Withdraw = "withdraw(uint256,address)",
	UpdateWhitelist = "updatewhitelist(address,bool)"
}


pub struct ExamplePrecompile;
	
impl Precompile for ExamplePrecompile {
	fn execute(
        input: &[u8], 
        target_gas: Option<u64>, 
        context: &Context,
		is_static: bool,
    ) -> Result<PrecompileOutput, PrecompileFailure>{

		let mut gasometer = Gasometer::new(target_gas);
		let gasometer = &mut gasometer;

		let (mut input, selector) = EvmDataReader::new_with_selector(gasometer, input)
		.unwrap_or_else(|_| (EvmDataReader::new(input), Action::Deposit));
		let input = &mut input;

		gasometer.check_function_modifier(
			context,
			is_static,
			match selector {
				Action::Withdraw | Action::UpdateWhitelist => {
				 FunctionModifier::NonPayable 
				} 

				Action::Deposit => FunctionModifier::Payable,
			} 
		)?;

		match selector {
			Action::Deposit => Self::deposit(input, gasometer, context),
			Action::Withdraw => Self::withdraw(input, gasometer, context),
			Action::UpdateWhitelist => Self::updatewhitelist(input, gasometer, context)
		}
	}
}

impl ExamplePrecompile {

	fn deposit(
		_: &mut EvmDataReader,
		gasometer: &mut Gasometer,
		context: &Context
	) -> EvmResult<PrecompileOutput> {
		
		//logic

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: Default::default(),
			logs: Default::default()
		})

	}

	fn withdraw(
		_: &mut EvmDataReader,
		gasometer: &mut Gasometer,
		context: &Context
	)-> EvmResult<PrecompileOutput> {

		//logic

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: Default::default(),
			logs: Default::default()
		})
	}

	fn updatewhitelist(
		_: &mut EvmDataReader,
		gasometer: &mut Gasometer,
		context: &Context
	)-> EvmResult<PrecompileOutput> {

		//logic

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output: Default::default(),
			logs: Default::default()
		})
	}
}
