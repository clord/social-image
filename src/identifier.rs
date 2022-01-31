use rand::{self, Rng};
use sha2::{Digest, Sha256};

pub struct FileId(String);

impl FileId {
    pub fn from_contents(source: &[u8]) -> FileId {
        let mut hasher = Sha256::new();
        hasher.update(source);
        let hash_bytes = hasher.finalize();
        FileId(bs58::encode(hash_bytes).into_string())
    }

    pub fn new(source: &[u8]) -> FileId {
        let mut rng = rand::thread_rng();

        let mut hasher = Sha256::new();
        for _ in 0..4 {
            hasher.update(rng.gen::<[u8; 8]>())
        }

        hasher.update(source);
        let hash_bytes = hasher.finalize();

        FileId(bs58::encode(hash_bytes).into_string())
    }

    pub fn dir(&self) -> &str {
        &self.0[0..2]
    }

    pub fn name(&self) -> &str {
        &self.0[2..]
    }
}
