//! IO for reading the configs (spec and lockfile)

use std::{
    io::{Cursor, Result},
    path::{Path, PathBuf},
};

use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWrite, BufReader, BufStream},
};

/// The 'a allows
pub trait ConfigIO<'a> {
    type R: AsyncRead;
    type RW: AsyncRead + AsyncWrite;

    async fn spec_file_ro(&self) -> Result<Self::R>;
    async fn lock_file_rw(&'a mut self) -> Result<Self::RW>;
}

pub struct Real {
    spec_path: PathBuf,
    lock_path: PathBuf,
}

impl Real {
    const SPEC_FILE_NAME: &'static str = "artificer.toml";
    const LOCK_FILE_NAME: &'static str = "artificer.lock";

    pub fn new(dir: &Path) -> Self {
        Self {
            spec_path: dir.join(Self::SPEC_FILE_NAME),
            lock_path: dir.join(Self::LOCK_FILE_NAME),
        }
    }
}

impl<'a> ConfigIO<'a> for Real {
    type R = BufReader<File>;
    type RW = BufStream<File>;

    async fn spec_file_ro(&self) -> Result<Self::R> {
        Ok(BufReader::new(File::open(&self.spec_path).await?))
    }

    async fn lock_file_rw(&'a mut self) -> Result<Self::RW> {
        Ok(BufStream::new(
            File::options()
                .write(true)
                .read(true)
                .open(&self.lock_path)
                .await?,
        ))
    }
}

struct Mock<'m> {
    spec_file: &'m [u8],
    lock_file: &'m mut [u8],
}

impl<'a, 'm: 'a> ConfigIO<'a> for Mock<'m> {
    // TODO: Do we need to introduce a 'b instead?
    type R = &'a [u8];
    type RW = Cursor<&'a mut [u8]>;

    async fn spec_file_ro(&self) -> Result<Self::R> {
        Ok(self.spec_file)
    }

    async fn lock_file_rw(&'a mut self) -> Result<Self::RW> {
        Ok(Cursor::new(self.lock_file))
    }
}

// This test mostly exists just to ensure that the lifetimes all compile correctly.
#[cfg(test)]
mod test {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test() {
        let mut spec = vec![0; 4];
        let mut lock = vec![0; 4];
        let mut m = Mock {
            spec_file: &mut spec,
            lock_file: &mut lock,
        };
        {
            let mut writer = m.lock_file_rw().await.unwrap();
            writer.write_all(b"foo").await.unwrap();
        }
        assert_eq!(m.lock_file, b"foo\0");
        {
            let mut reader = m.spec_file_ro().await.unwrap();
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            assert_eq!(bytes, m.spec_file);
        }
    }
}
