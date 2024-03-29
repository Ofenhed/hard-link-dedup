use async_trait::async_trait;
use std::{hash::Hash, io::Result, path::Path};
use tokio::fs::DirEntry;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
#[cfg(feature = "stable")]
mod windows;
#[cfg(windows)]
#[cfg(feature = "stable")]
pub use self::windows::*;
#[cfg(windows)]
#[cfg(not(feature = "stable"))]
mod windows_unstable;
#[cfg(windows)]
#[cfg(not(feature = "stable"))]
pub use self::windows_unstable::*;

pub trait FileLinkBackend {
  type StorageUid: Eq + Send + Hash;
  type FileId: Eq + Send + Hash;
  fn get_storage_uid(&self) -> Self::StorageUid;
  fn get_file_id(&self) -> Self::FileId;
  fn get_file_uid(&self) -> (Self::StorageUid, Self::FileId) {
    (self.get_storage_uid(), self.get_file_id())
  }
  fn same_storage(&self, other: &Self) -> bool {
    let (file1_uid, file2_uid) = (self.get_storage_uid(), other.get_storage_uid());
    file1_uid == file2_uid
  }
  fn same_file(&self, other: &Self) -> bool {
    let (file1_storage, file2_storage, file1_uid, file2_uid) = (
      self.get_storage_uid(),
      other.get_storage_uid(),
      self.get_file_id(),
      other.get_file_id(),
    );
    (file1_storage, file1_uid) == (file2_storage, file2_uid)
  }
}

#[async_trait]
pub trait FileBackend
where
  Self: Sized,
{
  type Metadata: FileLinkBackend + Send;
  async fn link_metadata(self) -> Result<Self::Metadata>;
}

pub async fn read_link_metadata<'a>(from: impl AsRef<Path> + 'a) -> Result<CurrentFileLinkBackend> {
  from.as_ref().link_metadata().await
}

pub type CurrentFileLinkBackend = <&'static DirEntry as FileBackend>::Metadata;
pub type StorageUid = <CurrentFileLinkBackend as FileLinkBackend>::StorageUid;
pub type FileId = <CurrentFileLinkBackend as FileLinkBackend>::FileId;
