use std::io;
use std::io::ErrorKind;

use async_std::{
    net::{Shutdown, TcpStream},
    prelude::*,
};
use async_trait::async_trait;

use crate::transport::{AsyncRead, AsyncReadHalf, AsyncWrite, AsyncWriteHalf, TAsyncIoChannel};

#[derive(Debug, Default)]
pub struct TAsyncTcpChannel {
    stream: Option<TcpStream>,
}

impl TAsyncTcpChannel {
    /// Create a `TAsyncTcpChannel` that wraps an existing `TcpStream`.
    ///
    /// The passed-in stream is assumed to have been opened before being wrapped
    /// by the created `TAsyncTcpChannel` instance.
    pub fn with_stream(stream: TcpStream) -> TAsyncTcpChannel {
        TAsyncTcpChannel {
            stream: Option::Some(stream),
        }
    }

    /// close a tcp channel
    pub fn close(&mut self) {
        if let Some(ref mut s) = self.stream {
            s.shutdown(Shutdown::Both).unwrap();
        };
    }
}

impl TAsyncIoChannel for TAsyncTcpChannel {
    fn split(&self) -> crate::Result<(AsyncReadHalf<Self>, AsyncWriteHalf<Self>)>
    where
        Self: Sized,
    {
        let read_half = AsyncReadHalf::new(TAsyncTcpChannel {
            stream: self.stream.clone(),
        });
        let write_half = AsyncWriteHalf::new(TAsyncTcpChannel {
            stream: self.stream.clone(),
        });
        Result::Ok((read_half, write_half))
    }
}

#[async_trait]
impl AsyncRead for TAsyncTcpChannel {
    async fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut s) = self.stream {
            s.read(b).await
        } else {
            Err(io::Error::new(
                ErrorKind::NotConnected,
                "tcp endpoint not connected",
            ))
        }
    }
}

#[async_trait]
impl AsyncWrite for TAsyncTcpChannel {
    async fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        // println!("in {:?}", b);
        if let Some(ref mut s) = self.stream {
            s.write(b).await
        } else {
            Err(io::Error::new(
                ErrorKind::NotConnected,
                "tcp endpoint not connected",
            ))
        }
    }

    async fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut s) = self.stream {
            s.flush().await
        } else {
            Err(io::Error::new(
                ErrorKind::NotConnected,
                "tcp endpoint not connected",
            ))
        }
    }
}
