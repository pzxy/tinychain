use clap::{Parser, Subcommand};
use log::info;

mod database;
mod node;
mod utils;
mod wallet;

/// The command of tiny-chain
#[derive(Debug, Parser)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// Creates a new account with a new set of a elliptic-curve Private + Public keys
    NewAccount {
        /// the node data dir where the account will be stored
        #[arg(short, long, default_value_t = String::from("./db/"))]
        datadir: String,
    },
    /// Launches the node
    Run {
        /// the node data dir where the DB will/is stored
        #[arg(short, long, default_value_t = String::from("./db/"))]
        datadir: String,
        /// exposed IP for communication with peers
        #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
        ip: String,
        /// exposed HTTP port for communication with peers
        #[arg(short, long, default_value_t = 8000)]
        port: u16,
        /// miner account of this node to receive block rewards
        #[arg(short, long)]
        miner: String,
    },
}

fn main() {
    env_logger::init();

    // 解析命令行参数
    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::NewAccount { datadir } => {
            wallet::init_keystore_dir(&datadir);
            let acc = wallet::new_account().unwrap();
            info!("New account created: {:?}", acc);
            info!("Saved in: {}", wallet::get_keystore_dir());
        }
        SubCommand::Run {
            datadir,
            ip,
            port,
            miner,
        } => {
            wallet::init_keystore_dir(&datadir);
            database::init_database_dir(&datadir);
            node::run(&ip, port, &miner).unwrap();
        }
    }
}
