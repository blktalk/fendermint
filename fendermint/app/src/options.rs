// Copyright 2022-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use num_traits::Num;
use tracing::Level;

use fvm_shared::{bigint::BigInt, econ::TokenAmount, version::NetworkVersion};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Options {
    /// Set a custom directory for configuration files.
    ///
    /// By default the application will try to find where the config directory is.
    #[arg(short, long, value_name = "DIR")]
    pub config_dir: Option<PathBuf>,

    /// Optionally override the default configuration.
    #[arg(short, long, default_value = "dev")]
    pub mode: String,

    /// Turn debugging information on.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

impl Options {
    pub fn tracing_level(&self) -> Level {
        match self.debug {
            0 => Level::ERROR,
            1 => Level::WARN,
            2 => Level::INFO,
            3 => Level::DEBUG,
            _ => Level::TRACE,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the [`App`], listening to ABCI requests from Tendermint.
    Run(RunArgs),
    /// Generate a new Secp256k1 key pair and export them to files in base64 format.
    Keygen(KeygenArgs),
    /// Subcommands related to the construction of Genesis files.
    Genesis(GenesisArgs),
}

#[derive(Subcommand, Debug)]
pub enum GenesisCommands {
    /// Create a new Genesis file, with accounts and validators to be added later.
    New(GenesisNewArgs),
    /// Add an account to the genesis file.
    AddAccount(GenesisAddAccountArgs),
    /// Add a multi-sig account to the genesis file.
    AddMultisig(GenesisAddMultisigArgs),
    /// Add a validator to the genesis file.
    AddValidator(GenesisAddValidatorArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs;

#[derive(Args, Debug)]
pub struct KeygenArgs {
    /// Name used to distinguish the files from other exported keys.
    #[arg(long, short)]
    pub name: String,
    /// Directory to export the key files to; it must exist.
    #[arg(long, short, default_value = ".")]
    pub out_dir: PathBuf,
}

#[derive(Args, Debug)]
pub struct GenesisArgs {
    /// Path to the genesis JSON file.
    #[arg(long, short)]
    pub genesis_file: PathBuf,

    #[command(subcommand)]
    pub command: GenesisCommands,
}

#[derive(Args, Debug)]
pub struct GenesisNewArgs {
    /// Name of the network and chain.
    #[arg(long, short = 'n')]
    pub network_name: String,
    /// Network version, governs which set of built-in actors to use.
    #[arg(long, short = 'v', default_value = "18", value_parser = parse_network_version)]
    pub network_version: NetworkVersion,
    /// Base fee for running transactions in atto.
    #[arg(long, short = 'f', value_parser = parse_token_amount)]
    pub base_fee: TokenAmount,
}

#[derive(Args, Debug)]
pub struct GenesisAddAccountArgs {
    /// Path to the Secp256k1 public key exported in base64 format.
    #[arg(long, short)]
    pub public_key: PathBuf,
    /// Initial balance in atto.
    #[arg(long, short, value_parser = parse_token_amount)]
    pub balance: TokenAmount,
}

#[derive(Args, Debug)]
pub struct GenesisAddMultisigArgs {
    /// Path to the Secp256k1 public key exported in base64 format, one for each signatory.
    #[arg(long, short)]
    pub public_key: Vec<PathBuf>,
    /// Initial balance in atto.
    #[arg(long, short, value_parser = parse_token_amount)]
    pub balance: TokenAmount,
    /// Number of signatures required.
    #[arg(long, short)]
    pub threshold: u64,
    /// Linear unlock duration in block heights.
    #[arg(long, short = 'd')]
    pub vesting_duration: u64,
    /// Linear unlock start block height.
    #[arg(long, short = 's')]
    pub vesting_start: u64,
}

#[derive(Args, Debug)]
pub struct GenesisAddValidatorArgs {
    /// Path to the Secp256k1 public key exported in base64 format.
    #[arg(long, short)]
    pub public_key: PathBuf,
    /// Voting power.
    #[arg(long, short = 'v')]
    pub power: u64,
}

fn parse_network_version(s: &str) -> Result<NetworkVersion, String> {
    let nv: u32 = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a network version"))?;
    if nv >= 18 {
        Ok(NetworkVersion::from(nv))
    } else {
        Err("the minimum network version is 18".to_owned())
    }
}

fn parse_token_amount(s: &str) -> Result<TokenAmount, String> {
    BigInt::from_str_radix(s, 10)
        .map_err(|e| format!("not a token amount: {e}"))
        .map(TokenAmount::from_atto)
}