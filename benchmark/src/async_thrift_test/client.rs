// only for test!!!
//

use async_std::net::TcpStream;
//use async_std::sync::Receiver;

use crate::async_thrift_test::tutorial::{CalculatorSyncClient, TCalculatorSyncClient};
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocol, TAsyncBinaryOutputProtocol};
use async_thrift::transport::async_buffered::{
    TAsyncBufferedReadTransport, TAsyncBufferedWriteTransport,
};

use async_thrift::transport::async_socket::TAsyncTcpChannel;
use async_thrift::transport::TAsyncIoChannel;

pub async fn run_client(addr: String, loop_num: i32) -> async_thrift::Result<Vec<i64>> {
    // time
    // let start = time::now();

    let stream = TcpStream::connect(addr.as_str()).await?;

    let mut c = TAsyncTcpChannel::with_stream(stream);

    let (i_chan, o_chan) = c.split().unwrap();

    let i_prot = TAsyncBinaryInputProtocol::new(TAsyncBufferedReadTransport::new(i_chan), true);
    let o_prot = TAsyncBinaryOutputProtocol::new(TAsyncBufferedWriteTransport::new(o_chan), true);

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);

    let mut time_array = Vec::with_capacity(loop_num as usize);

    for _ in 0..loop_num {
        let before = time::Instant::now();
        client.ping().await?;
        let end = time::Instant::now();
        time_array.push((end - before).whole_nanoseconds() as i64);
    }

    c.close();

    Ok(time_array)
}
