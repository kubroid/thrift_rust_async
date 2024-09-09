use std::borrow::Borrow;
use std::env;

use tabled::builder::Builder;

#[derive(Debug)]
pub struct TestResult {
    pub mode: String,
    pub call_num: i64,
    pub tot_time: i64,
    pub data: Vec<i64>,
}

impl TestResult {
    pub fn as_vec(&self) -> Vec<String> {
        let mut rec = Vec::<String>::new();
        rec.push(self.mode.clone());
        rec.push(format!("{}ms", self.tot_time));
        rec.push(format!("{}", (1000 * self.call_num / self.tot_time)));
        for t in &self.data {
            rec.push(format!("{}us", t / 1000));
        }

        rec
    }
}

/// print time result in md table
pub fn format_result(data: Vec<TestResult>) -> String {
    let header = [
        "mode",
        "total time",
        "query per second",
        "avg time",
        "per 50 time",
        "per 90 time",
        "per 95 time",
        "per 99 time",
        "per 99.9 time",
        "max time",
    ];

    let mut table = Builder::default();
    table.push_record(header);
    for r in &data {
        table.push_record(r.as_vec())
    }

    table.build().to_string()
}

/// print config result in md table
pub fn format_config(thread_num: i32, loop_num: i32) -> String {
    format!(
        "###config
|  thread num   | loop num  | total call |
|  -----------  | --------  | ---------- |
|      {}      |    {}    |    {}    |",
        format_i32(thread_num),
        format_i32(loop_num),
        format_i32(thread_num * loop_num)
    )
}

/// format i32 for human
/// # Examples
///
/// ```no_run
/// let i = 1000000;
/// assert_eq!("1_00_000", format_i32(i))
/// ```
fn format_i32(mut i: i32) -> String {
    let mut res = String::new();
    let mut count = 0;
    while i > 0 {
        if count == 3 {
            res.insert(0, '_');
            count = 0;
        }
        count += 1;
        let last = i % 10;
        i /= 10;
        res.insert_str(0, last.to_string().as_str());
    }

    res
}

/// print welcome message
pub fn print_welcome() {
    println!("******************************************");
    println!("*        E-01 benchmark for rust rpc     *");
    println!("*             Version : 0.1.0            *");
    println!("******************************************");
    println!("---------------------------   Benchmark Start! --------------------------");
}

/// print benchmark result
pub fn print_result(output: Vec<Option<TestResult>>) {
    println!();
    println!("---------------------------   Benchmark Finished! --------------------------");
    println!();

    let o: Vec<TestResult> = output.into_iter().flatten().collect();

    println!("{}", format_result(o));
}

pub fn handle_time(time_arrays: Vec<Vec<i64>>) -> Vec<i64> {
    let mut sum = 0;
    let mut count = 0;
    let mut times: Vec<i64> = Vec::new();
    for time_array_result in time_arrays {
        let time_array: &Vec<i64> = time_array_result.borrow();
        for time in time_array {
            times.push(*time);
            sum += time;
            count += 1;
        }
    }

    times.sort();
    let res: Vec<i64> = vec![
        // avg
        sum / count,
        // per 50
        times[times.len() / 2],
        // per 90
        times[(times.len() / 10) * 9],
        // per 95
        times[(times.len() / 100) * 95],
        // per 99
        times[(times.len() / 100) * 99],
        // per 99.9
        times[(times.len() / 1000) * 999],
        // max time
        times[times.len() - 1],
    ];

    res
}

/// parse command line args
pub fn parse_args(args: &mut [String]) {
    let mut loc = 1000000;
    for s in env::args() {
        if loc == 1000000 {
            loc = 0;
        } else {
            args[loc] = s;
            loc += 1;
        }
    }
}
