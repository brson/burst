use St;

use bcore::boxed::Box;
use core::mem;
use core::fmt::Write;
use log::{self, MaxLogLevelFilter, Log, SetLoggerError,
          LogMetadata, LogRecord, LogLevelFilter, LogLevel};
use io;

pub fn init(st: &St) {
    set_logger(|filter| {
        // FIXME: Set log level filter correctly
        filter.set(LogLevelFilter::Trace);
        Box::new(st, Logger(filter))
    }).unwrap(); // FIXME: error handling

    // FIXME: this never calls shutdown_logger_raw
}

fn set_logger<M>(make_logger: M) -> Result<(), SetLoggerError>
        where M: FnOnce(MaxLogLevelFilter) -> Box<Log> {
    unsafe {
        log::set_logger_raw(|max_level| {
            mem::transmute(make_logger(max_level))
        })
    }
}

struct Logger(MaxLogLevelFilter);

impl Log for Logger {
    fn enabled(&self, _metadata: &LogMetadata) -> bool {
        unimplemented!()
    }

    fn log(&self, record: &LogRecord) {

        let args = record.args();

        let level = record.level();
        let _target = record.target();

        let module_path = record.location().module_path();
        let _file = record.location().file();
        let _line = record.location().line();

        let module_name = module_path.rfind("::").map(|i| {
            &module_path[i + 2 ..]
        }).unwrap_or(module_path);

        let crate_name = module_path.find("::").map(|i| {
            &module_path[ .. i]
        }).unwrap_or(module_path);

        let mut stderr = io::stdio::stderr();

        let level = match level {
            LogLevel::Error => "ERROR",
            LogLevel::Warn =>  " warn",
            LogLevel::Info =>  " info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        };

        if module_name == crate_name {
            let _ = writeln!(
                &mut stderr,
                "{}| {}: {}",
                level, crate_name, args);
        } else {
            let _ = writeln!(
                &mut stderr,
                "{}| {}/{}: {}",
                level, crate_name, module_name, args);
        }

        /*let _ = writeln!(&mut stderr,
                         "      |{} {}:{}",
                         module_path,
                         file,
                         line);*/
                         
    }
}
