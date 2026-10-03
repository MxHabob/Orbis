pub mod filesystem;
pub mod resolver;
pub mod s3;

pub use filesystem::FilesystemStorageAdapter;
pub use resolver::StorageResolver;
pub use s3::{S3Config, S3StorageAdapter};
