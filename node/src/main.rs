//! Substrate Node Template CLI library.
#![warn(missing_docs)]
#![allow(non_snake_case)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
