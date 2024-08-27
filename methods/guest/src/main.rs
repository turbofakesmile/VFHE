use risc0_zkvm::guest::env;
use tfhe::boolean::prelude::*;
// use tfhe::FheBool;

fn main() {
    println!("Init");
    // let raw_key_bin: Vec<u8> = env::read();
    // println!("Read raw binary key");
    // let (raw_key, wopbs_key): (RawKey, Option<WopbsKey>) = bincode::deserialize(&raw_key_bin)
    //     .expect("serde error");
    // println!("Deserialized raw key");
    // // let raw_key: RawKey = env::read();
    // // println!("Read raw key");
    // // let wopbs_key: Option<WopbsKey> = env::read();
    // // println!("Read wopbs key");
    // let server_key = ServerKey::from_raw_parts(raw_key, wopbs_key);

    // let server_key: ServerKey = env::read();
    // println!("Read server key");
    // // set_server_key(server_key);
    // println!("Set server key");
    //
    // let enc_a: FheBool = env::read();
    // let enc_b: FheBool = env::read();
    // println!("Read inputs");
    //
    // let res = enc_a && enc_b;
    // println!("Calculated result");
    //
    // env::commit(&res);
    // println!("Committed output");
}
