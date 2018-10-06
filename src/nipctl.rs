#[macro_use]
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate byteorder;
extern crate futures;
extern crate git2;
extern crate hyper;
extern crate ipfs_api;
extern crate serde_cbor;
extern crate tokio_core;

mod constants;
mod nip_index;
mod nip_ref;
mod util;

use byteorder::{BigEndian, WriteBytesExt};
use git2::Repository;
use ipfs_api::IpfsClient;
use log::LevelFilter;
use tokio_core::reactor::Core;

use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufReader, Cursor, Write},
    sync::{Arc, Mutex},
    thread,
};

use constants::{NIP_MAGIC, NIP_PROTOCOL_VERSION};
use nip_index::NIPIndex;
use nip_ref::NIPRef;
use util::gen_nip_header;

/// A simple binary for managing nip remotes
pub fn main() {
    util::init_logging(LevelFilter::Info);

    info!("Generating a new garbage index");

    let mut buf = gen_nip_header(None).unwrap();

    info!("Header: {:?}", buf.clone());

    let nip_ref = NIPRef::new(
        "refs/heads/master".to_owned(),
        "529885ae94597ffdc9c8adae9b643f103c590b88".to_owned(),
        "QmejvEPop4D7YUadeGqYWmZxHhLc4JBUCzJJHWMzdcMe2y".to_owned(),
    ).unwrap();

    let mut refs = BTreeSet::new();
    refs.insert(nip_ref);

    let idx = NIPIndex {
        refs,
        prev_idx_hash: None,
    };

    buf.extend_from_slice(&serde_cbor::to_vec(&idx).unwrap());
    drop(idx);
    info!("Full serialized bytefield: {:?}", buf.clone());

    let mut ipfs = IpfsClient::default();

    let req = ipfs.add(Cursor::new(buf));
    let mut event_loop = Core::new().unwrap();
    let response = event_loop.run(req).unwrap();
    info!("Response: {:?}", response);
}
