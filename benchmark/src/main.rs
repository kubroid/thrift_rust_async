//extern crate jemallocator;

//#[global_allocator]
//static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use async_std::task;
use futures::future::*;

// async use
use util::TestResult;

use crate::util::{handle_time, parse_args};

mod async_thrift_test;

// sync use
mod async_thrift_test_tokio;
mod sync_thrift_test;

// util
mod util;

// const
const RUN_CLIENT: usize = 0;
const RUN_SERVER: usize = 1;
const RUN_SYNC: usize = 2;
const RUN_ASYNC: usize = 3;
const RUN_ASYNC_TOKIO: usize = 4;
const THREAD_NUM: usize = 5;
const LOOP_NUM: usize = 6;
const ADDR: usize = 7;

///
const DEFAULT_RUN_CLIENT: &str = "true";
const DEFAULT_RUN_SERVER: &str = "true";
const DEFAULT_RUN_SYNC: &str = "true";
const DEFAULT_RUN_ASYNC: &str = "true";
const DEFAULT_RUN_ASYNC_TOKIO: &str = "true";
const DEFAULT_THREAD_NUM: &str = "10";
const DEFAULT_LOOP_NUM: &str = "10000";
const DEFAULT_ADDR: &str = "127.0.0.1:9090";

// run sync server and client
fn run_sync_both(args: Arc<Vec<String>>) -> Option<TestResult> {
    println!("begin sync benchmark...");

    let mut ret = None;

    if args[RUN_SERVER] == *"true" {
        // start server
        let addr = Clone::clone(&args[ADDR]);

        let server = thread::spawn(move || sync_thrift_test::server::run(addr.as_str()));
        // we need to wait the server to run
        thread::sleep(Duration::from_secs(2));

        if args[RUN_CLIENT] != *"true" {
            println!("server is online");
            let _ = server.join();

            return ret;
        }
    }

    if args[RUN_CLIENT] == *"true" {
        // time clock start here
        let start = time::Instant::now();

        // build client thread
        let mut list = Vec::new();

        for _i in 0..args[THREAD_NUM].parse::<i32>().unwrap() {
            // to ensure tcp sync queue is enough
            let stream = std::net::TcpStream::connect(args[ADDR].as_str()).unwrap();
            // build client
            let loop_num = args[LOOP_NUM].parse::<i32>().unwrap();
            //
            list.push(thread::spawn(move || {
                sync_thrift_test::client::run(stream, loop_num)
            }));
        }

        // to collect time result from client
        let mut res = Vec::new();
        for task in list {
            res.push(task.join().unwrap().unwrap());
        }

        // time clock end here;
        let end = time::Instant::now();

        // handle raw time result to statistic
        let time_statistic = handle_time(res);
        ret = Some(TestResult {
            mode: "sync".into(),
            call_num: args[THREAD_NUM].parse::<i64>().unwrap()
                * args[LOOP_NUM].parse::<i64>().unwrap(),
            tot_time: (end - start).whole_milliseconds() as i64,
            data: time_statistic,
        });
    }

    println!("sync finished!");

    ret
}

// run async server and client
async fn run_async_both(args: Arc<Vec<String>>) -> Option<TestResult> {
    println!("begin async benchmark...");

    let mut server = None;
    let addr = &args[ADDR];

    let mut ret = None;

    if args[RUN_SERVER] == *"true" {
        server = Some(async_std::task::spawn(
            async_thrift_test::server::run_server(Clone::clone(addr)),
        ));
        if args[RUN_CLIENT] != *"true" {
            println!("server is online");
            server.unwrap().await;

            return ret;
        }
    }
    if args[RUN_CLIENT] == *"true" {
        let loop_num = args[LOOP_NUM].parse::<i32>().unwrap();
        let coroutine_num = args[THREAD_NUM].parse::<i32>().unwrap();

        // time
        let mut list = Vec::new();

        for _i in 0..coroutine_num {
            // build client
            list.push(async_std::task::spawn(
                async_thrift_test::client::run_client(Clone::clone(addr), loop_num),
            ));
        }

        println!("all jobs generated!");

        let start = time::Instant::now();

        // time clock start here
        let raw_time_result = join_all(list).await;

        // time clock end here;
        let end = time::Instant::now();

        // to collect time result from client
        let mut res = Vec::new();
        for task in raw_time_result {
            res.push(task.unwrap());
        }

        // handle raw time result to statistic
        let time_statistic = handle_time(res);

        ret = Some(TestResult {
            mode: "async".into(),
            call_num: (coroutine_num * loop_num) as i64,
            tot_time: (end - start).whole_milliseconds() as i64,
            data: time_statistic,
        })
    }

    if args[RUN_SERVER] == *"true" {
        server.unwrap().cancel().await;
    }

    println!("async finished!");

    ret
}

// run async server and client
async fn run_async_tokio_both(args: Arc<Vec<String>>) -> Option<TestResult> {
    println!("begin async tokio benchmark...");

    let addr = &args[ADDR];
    let mut ret = None;

    if args[RUN_SERVER] == *"true" {
        let server = tokio::task::spawn(async_thrift_test_tokio::server::run_server(Clone::clone(
            addr,
        )));
        if args[RUN_CLIENT] != *"true" {
            println!("server is online");
            let _ = server.await;

            return ret;
        }
    }

    if args[RUN_CLIENT] == *"true" {
        // time
        let mut list = Vec::new();

        let loop_num = args[LOOP_NUM].parse::<i32>().unwrap();
        let coroutine_num = args[THREAD_NUM].parse::<i32>().unwrap();

        for _i in 0..coroutine_num {
            // build client
            list.push(tokio::task::spawn(
                async_thrift_test_tokio::client::run_client(Clone::clone(addr), loop_num),
            ));
        }

        let start = time::Instant::now();
        // time clock start here
        let raw_time_result = join_all(list).await;

        // time clock end here;
        let end = time::Instant::now();

        // to collect time result from client
        let mut res = Vec::new();
        for task in raw_time_result {
            res.push(task.unwrap().unwrap());
        }

        // handle raw time result to statistic
        let time_statistic = handle_time(res);

        ret = Some(TestResult {
            mode: "async tokio".into(),
            call_num: (coroutine_num * loop_num) as i64,
            tot_time: (end - start).whole_milliseconds() as i64,
            data: time_statistic,
        })
    }

    println!("async tokio finished!");

    ret
}

fn main() {
    let mut args: Vec<String> = vec![
        String::from(DEFAULT_RUN_CLIENT),
        String::from(DEFAULT_RUN_SERVER),
        String::from(DEFAULT_RUN_SYNC),
        String::from(DEFAULT_RUN_ASYNC),
        String::from(DEFAULT_RUN_ASYNC_TOKIO),
        String::from(DEFAULT_THREAD_NUM),
        String::from(DEFAULT_LOOP_NUM),
        String::from(DEFAULT_ADDR),
    ];

    parse_args(&mut args);

    println!("{:?}", &args);

    let mut output = Vec::<Option<TestResult>>::new();

    util::print_welcome();

    println!(
        "{}",
        util::format_config(
            args[THREAD_NUM].parse::<i32>().unwrap(),
            args[LOOP_NUM].parse::<i32>().unwrap(),
        )
    );

    let arc_args = Arc::new(args);
    // async part
    if arc_args[RUN_ASYNC] == *"true" {
        output.push(task::block_on(run_async_both(Arc::clone(&arc_args))));
    }

    // async tokio part
    if arc_args[RUN_ASYNC_TOKIO] == *"true" {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        output.push(rt.block_on(run_async_tokio_both(Arc::clone(&arc_args))));
        rt.shutdown_background();
    }

    // sync part
    if arc_args[RUN_SYNC] == *"true" {
        output.push(run_sync_both(Arc::clone(&arc_args)));
    }
    util::print_result(output);
}
