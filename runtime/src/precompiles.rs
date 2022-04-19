use pallet_evm::{Context, Precompile, PrecompileResult, PrecompileSet, AddressMapping};
use sp_core::H160;
use sp_std::marker::PhantomData;


use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};

use prepay_gas::TestPC1;


#[derive(Debug, Clone, Copy)]
pub struct MyChainPrecompiles<R>(PhantomData<R>);

impl<R> MyChainPrecompiles<R>
where
    R: pallet_evm::Config,
{
    pub fn new() -> Self {
        Self(Default::default())
    }
    pub fn used_addresses() -> impl Iterator<Item = R::AccountId> {
        sp_std::vec![1, 2, 3, 4, 5, 51, 1024, 1025]
            .into_iter()
            .map(|x| R::AddressMapping::into_account_id(hash(x)))
    }
}


impl<R> PrecompileSet for MyChainPrecompiles<R>
where
    R: pallet_evm::Config,
{
    fn execute(
        &self,
        address: H160,
        input: &[u8],
        target_gas: Option<u64>,
        context: &Context,
        is_static: bool,
    ) -> Option<PrecompileResult> {
        match address {
            // Ethereum precompiles :
            a if a == hash(1) => Some(ECRecover::execute(input, target_gas, context)),
            a if a == hash(2) => Some(Sha256::execute(input, target_gas, context)),
            a if a == hash(3) => Some(Ripemd160::execute(input, target_gas, context)),
            a if a == hash(4) => Some(Identity::execute(input, target_gas, context)),
            a if a == hash(5) => Some(Modexp::execute(input, target_gas, context)),

            // MyChain specific :
            a if a == hash(51) => Some(TestPC1::execute(
                 input, target_gas, context, is_static,
            )),

            // Non-MyChain specific nor Ethereum precompiles :
            a if a == hash(1024) => {
                Some(Sha3FIPS256::execute(input, target_gas, context))
            }
            a if a == hash(1025) => Some(ECRecoverPublicKey::execute(
                input, target_gas, context,
            )),
            _ => None,
        }
    }

    fn is_precompile(&self, address: H160) -> bool {
		Self::used_addresses().any(|x| x == R::AddressMapping::into_account_id(address))
    }
}

fn hash(a: u64) -> H160 {
    H160::from_low_u64_be(a)
}