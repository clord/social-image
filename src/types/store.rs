
use super::{SvgId, SvgDescription, Result};

/// keep svg elementsin some sort of persistent storage
#[async_trait]
pub trait Store : Sync + Send {
    /// save some bytes with a certain mime type to the store
    async fn save(&mut self, package: SvgDescription) -> Result<SvgId>;

    /// retreive bytes from the store
    async fn load(&self, file_id: SvgId) -> Result<SvgDescription>;

    /// Delete a given fileid from store
    async fn delete(&mut self, file_id: SvgId) -> Result<()>;
}


