use async_trait::async_trait;
use std::{fs::File, hash::Hash, io::Result};
use tokio::join;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

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
pub trait FileBackend {
  type Metadata: FileLinkBackend + Send;
  async fn link_metadata(&self) -> Result<Self::Metadata>;
  async fn same_file(&self, other: &Self) -> Result<bool> {
    let (file1, file2) = join!(self.link_metadata(), other.link_metadata());
    Ok(file1?.get_file_uid() == file2?.get_file_uid())
  }
}

pub type StorageUid = <<File as FileBackend>::Metadata as FileLinkBackend>::StorageUid;
pub type FileId = <<File as FileBackend>::Metadata as FileLinkBackend>::FileId;
