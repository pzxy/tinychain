use anyhow::Result;
use ethers_core::{rand::thread_rng, types::Signature, utils::hash_message};
use ethers_signers::{LocalWallet, Signer};
use once_cell::sync::OnceCell;
use std::{fs, str::FromStr, path::PathBuf};

const PASSWORD: &str = "774411";
static KEYSTORE_DIR: OnceCell<String> = OnceCell::new();

pub fn init_keystore_dir(datadir: &str) {
    let mut dir = PathBuf::from(datadir);
    dir.push("keystore/");
    KEYSTORE_DIR.get_or_init(||dir.into_os_string().into_string().unwrap());
}

pub fn new_account() -> Result<String> {
    let dir = get_keystore_dir();
    fs::create_dir_all(dir)?;

    let (_, account) = LocalWallet::new_keystore(dir, &mut thread_rng(), PASSWORD, None)?;
    Ok(account)
}

pub fn sign(msg: &str, account: &str) -> Result<String> {
    let sig = get_wallet(account)?
        .sign_hash(hash_message(msg))
        .to_string();

    Ok(sig)
}

pub fn verify(msg: &str, sig: &str, account: &str) -> Result<()> {
    let wallet = get_wallet(account)?;
    let sig = Signature::from_str(sig)?;
    sig.verify(msg, wallet.address())?;
    Ok(())
}

pub fn get_keystore_dir() -> &'static str {
    KEYSTORE_DIR.get().unwrap()
}

fn get_wallet(account: &str) -> Result<LocalWallet> {
    let mut keypath = get_keystore_dir().to_owned();
    keypath.push_str(account);

    let wallet = LocalWallet::decrypt_keystore(&keypath, PASSWORD)?;
    Ok(wallet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystore_dir_can_only_be_initialized_once() {
        init_keystore_dir("/tmp/");
        assert_eq!("/tmp/keystore/", get_keystore_dir());

        init_keystore_dir("/another/dir/");
        assert_eq!("/tmp/keystore/", get_keystore_dir());
    }

    #[test]
    fn test_wallet() {
        init_keystore_dir("/tmp/");

        let acc = new_account().unwrap();
        let msg = "hello world";
        let sig = sign(msg, &acc).unwrap();
        assert!(verify(msg, &sig, &acc).is_ok());

        fs::remove_dir_all(get_keystore_dir()).unwrap();
    }
}
