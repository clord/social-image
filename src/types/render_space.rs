use rand::{self, Rng};
use sha2::{Digest, Sha256};
use std::{fs, path};

use super::Result;

/// Represent a place on the filesystem to render some content. when dropped, the location is deleted!
pub struct RenderSpace(path::PathBuf);

impl RenderSpace {
    pub fn new<P: AsRef<path::Path>>(store: P) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let mut hasher = Sha256::new();
        hasher.update(rng.gen::<[u8; 32]>());
        hasher.update(rng.gen::<[u8; 32]>());

        let hash_bytes = hasher.finalize();

        let name = bs58::encode(hash_bytes).into_string();
        let mut path = path::PathBuf::from(store.as_ref());
        path.push(&name[0..2]);
        path.push(&name[2..3]);
        path.push(&name[3..]);
        fs::create_dir_all(&path)?;
        info!("Rendering in {:?}", &path);
        Ok(RenderSpace(path))
    }
}

impl AsRef<path::Path> for RenderSpace {
    fn as_ref(&self) -> &path::Path {
        self.0.as_path()
    }
}

impl From<RenderSpace> for path::PathBuf {
    fn from(p: RenderSpace) -> path::PathBuf {
        p.0.clone()
    }
}

impl Drop for RenderSpace {
    fn drop(&mut self) {
        info!("Finished with render space {:?}", self.0);
        fs::remove_dir_all(&self.0).unwrap();
    }
}
