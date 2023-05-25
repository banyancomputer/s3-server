//! Block store traits.

use std::borrow::Cow;

use anyhow::{anyhow, Result};
// use async_trait::async_trait;
use hashbrown::HashMap;
use libipld::{cid::Version, Cid, IpldCodec};
use multihash::{Code, MultihashDigest};

// //--------------------------------------------------------------------------------------------------
// // Type Definitions
// //--------------------------------------------------------------------------------------------------

// /// For types that implement block store operations like adding, getting content from the store.
// #[async_trait(?Send)]
// pub trait BlockStore {
//     async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, Vec<u8>>>;
//     async fn put_block(&mut self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid>;
// }

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use wnfs::BlockStore;

/// An in-memory block store to simulate IPFS.
///
/// IPFS is basically a glorified HashMap.
#[derive(Debug, Default)]
pub struct MutexMemoryBlockStore(Arc<Mutex<HashMap<String, Vec<u8>>>>);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl MutexMemoryBlockStore {
    /// Creates a new in-memory block store.
    pub fn new() -> Self {
        Self::default()
    }
}

unsafe impl Send for MutexMemoryBlockStore {}
unsafe impl Sync for MutexMemoryBlockStore {}

#[async_trait]
impl BlockStore for MutexMemoryBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&mut self, bytes: Vec<u8>, codec: wnfs::ipld::IpldCodec) -> Result<Cid> {
        let hash = Code::Sha2_256.digest(&bytes);
        let cid = Cid::new(Version::V1, codec.into(), hash)?;

        self.0.insert((&cid).to_string(), bytes);

        Ok(cid)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block<'a>(&'a self, cid: &Cid<64>) -> Result<Cow<'a, Vec<u8>>> {
        let bytes = self
            .0
            .lock()
            .await
            .get(&cid.to_string())
            .ok_or(Err(anyhow!("CID not found in blockstore")));

        Ok(Cow::Borrowed(bytes))
    }
}
