pub mod filesystem;
pub mod s3;
pub mod resolver;

pub use filesystem::FilesystemStorageAdapter;
pub use s3::{S3StorageAdapter, S3Config};
pub use resolver::StorageResolver;
