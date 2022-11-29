mod block;
mod genesis;
mod state;
mod tx;

use std::path::PathBuf;

pub use block::*;
pub use genesis::*;
pub use state::*;
pub use tx::*;

use once_cell::sync::OnceCell;

static DATABASE_DIR: OnceCell<String> = OnceCell::new();
static GENESIS_PATH: OnceCell<String> = OnceCell::new();
static BLOCKDB_PATH: OnceCell<String> = OnceCell::new();

pub fn init_database_dir(datadir: &str) {
    let mut dir = PathBuf::from(datadir);
    dir.push("database/");

    let mut genesis_path = dir.clone();
    let mut blockdb_path = dir.clone();
    genesis_path.push("genesis.json");
    blockdb_path.push("block.db");

    DATABASE_DIR.get_or_init(|| dir.into_os_string().into_string().unwrap());
    GENESIS_PATH.get_or_init(|| genesis_path.into_os_string().into_string().unwrap());
    BLOCKDB_PATH.get_or_init(|| blockdb_path.into_os_string().into_string().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_dir_can_only_be_initialized_once() {
        init_database_dir("/tmp/");
        assert_eq!("/tmp/database/", DATABASE_DIR.get().unwrap());
        assert_eq!("/tmp/database/genesis.json", GENESIS_PATH.get().unwrap());
        assert_eq!("/tmp/database/block.db", BLOCKDB_PATH.get().unwrap());

        init_database_dir("/another/dir/");
        assert_eq!("/tmp/database/", DATABASE_DIR.get().unwrap());
        assert_eq!("/tmp/database/genesis.json", GENESIS_PATH.get().unwrap());
        assert_eq!("/tmp/database/block.db", BLOCKDB_PATH.get().unwrap());
    }
}
