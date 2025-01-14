use std::{error::Error, path::PathBuf};
use std::path::Path;

use kuska_handshake;
//use kuska_handshake::sodiumoxide::crypto::box_::{PublicKey, SecretKey};
use kuska_ssb::keystore::{self, OwnedIdentity};
use dirs_next;

use tokio::io::{self, BufWriter, AsyncWriteExt};
use tokio::fs::File;

use kuska_ssb::keystore::{
    JsonSSBSecret, CURVE_ED25519,
};
use serde_json::to_vec_pretty;

use kuska_ssb::crypto::ToSsbId;

use crate::ssb::TokioCompatFix;


// TODO: make error enum for this
// TODO: move this to generic file src dir
fn get_home_dir() -> Result<PathBuf, String>
{
    let home_dir: PathBuf = dirs_next::home_dir().expect("User home directory should be readable");
    return Ok(home_dir);
}

fn get_ssb_secret_path() -> Result<PathBuf, ()>
{
    if let Ok(mut home_dir) = get_home_dir() {
        home_dir.push(".ssb");
        return Ok(home_dir);
    }
    return Err(())
}

pub async fn get_ssb_id() -> Result<OwnedIdentity, String>
{
    let result = get_ssb_secret_path();
    if result == Err(()) {
        return Err("Failed to read ssbsecret directory.".to_owned());
    }

    let mut ssb_secret_p: PathBuf = result.unwrap();

    let ssb_secret_f = File::open(ssb_secret_p)
        .await
        .expect("Unable to open ssb secret file");

    let mut ssb_reader: BufReader<File> = BufReader::new(ssb_secret_f);

    let mut async_std_adapter: TokioCompatFix<&mut BufReader<File>> = TokioCompatFix { 
        0: &mut ssb_reader
    };

    let id_result = kuska_ssb::keystore::read_patchwork_config(&mut async_std_adapter).await;
    if id_result.is_err() {
        return Err("Failed to read ssbsecret directory.".to_owned());
    }

    return Ok(id_result.unwrap());
}


pub async fn first_time_id_gen()
{
    let kp_struct: OwnedIdentity = kuska_ssb::keystore::OwnedIdentity::create();

    let mut ssb_secret_p = get_ssb_secret_path()
        .expect("Unable to read home path");

    use tokio::fs;

    fs::create_dir(&ssb_secret_p).await
        .expect("Unable to create ssb secret path");
    ssb_secret_p.push("secret");

    let ssb_secret_f = File::create(ssb_secret_p)
        .await
        .expect("Unable to create ssb secret file");

    let mut ssb_secret_w: BufWriter<File> = BufWriter::new(ssb_secret_f);

    // use crate::ssb::TokioCompatFix;
    // let mut async_std_adapter: TokioCompatFix<&mut BufWriter<File>> = TokioCompatFix { 
    //     0: &mut ssb_secret_w
    // };

    write_patchwork_config_fixed(&kp_struct, &mut ssb_secret_w)
        .await
}


pub async fn write_patchwork_config_fixed(id: &OwnedIdentity, writer: &mut BufWriter<File>) 
{
    let json = JsonSSBSecret {
        id: id.id.clone(),
        curve: CURVE_ED25519.to_owned(),
        public: id.pk.to_ssb_id(),
        private: id.sk.to_ssb_id(),
    };
    let encoded = to_vec_pretty(&json)
        .expect("Unable to serialize json");

    writer.write(&encoded).await
        .expect("Unable to write to ssb secret file");

    writer.flush().await
        .expect("Unable to write to ssb secret file");
}


pub const GATE_NET_ID: [u8; 32] = [
    0x53,
    0x4d,
    0x61,
    0x72,
    0x41,
    0x79,
    0x70,
    0x4c,
    0x65,
    0x64,
    0x59,
    0x55,
    0x79,
    0x4f,
    0x78,
    0x6a,
    0x69,
    0x74,
    0x79,
    0x2b,
    0x68,
    0x74,
    0x72,
    0x7a,
    0x51,
    0x62,
    0x6f,
    0x5a,
    0x47,
    0x79,
    0x30,
    0x2f
];

pub const SSB_NET_ID: [u8; 32] = [
    0xd4,
    0xa1,
    0xcb,
    0x88,
    0xa6,
    0x6f,
    0x02,
    0xf8,
    0xdb,
    0x63,
    0x5c,
    0xe2,
    0x64,
    0x41,
    0xcc,
    0x5d,
    0xac,
    0x1b,
    0x08,
    0x42,
    0x0c,
    0xea,
    0xac,
    0x23,
    0x08,
    0x39,
    0xb7,
    0x55,
    0x84,
    0x5a,
    0x9f,
    0xfb
];