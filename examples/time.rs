use cantools::logging::CANDumpLog;
use std::time::Instant;

fn main() {
    let candump = CANDumpLog::open("candump/testdump_3.log").unwrap();

    let now = Instant::now();
    for _l in candump {
        // println!("{:?}", l);
    }
    let duration = now.elapsed();
    println!("{}", duration.as_secs_f32());
}
