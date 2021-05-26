use llog::Level;
use fast_log::appender::{FastLogRecord, LogAppender, RecordFormat};
use yansi::Paint;
use crate::config::{Settings, ExplorerLog};
use fast_log::error::LogError;
use fast_log::wait::FastLogWaitGroup;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{RollingType, FileSplitAppender};
use std::time::Duration;
use fast_log::filter::NoFilter;
use fast_log::init_custom_log;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, SeekFrom};
use fast_log::plugin::file::FileAppender;
use fast_log::consts::LogSize::MB;
use std::path::{Path, PathBuf};
use std::{env, fs};

struct ExplorerLogAppender {
    file: RefCell<File>,
    max_size: LogSize,
}

impl ExplorerLogAppender {
    pub fn new(log_file_path: PathBuf, max_size: LogSize) -> ExplorerLogAppender {
        Self {
            file: RefCell::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_file_path)
                    .unwrap(),
            ),
            max_size,
        }
    }
}


impl LogAppender for ExplorerLogAppender {
    fn do_log(&self, record: &FastLogRecord) {
        let data;

        let mut log_file = self.file.borrow_mut();


        let tar = Paint::blue("Explorer API").bold();
        match record.level {
            Level::Error => {
                data = format!(
                    "{} {} {} {} - {}  {}\n",
                    tar,
                    &record.now,
                    Paint::red(record.level).bold(),
                    record.module_path,
                    record.args,
                    record.format_line()
                );
            }
            Level::Warn => {
                data = format!(
                    "{} {} {} {} - {}  {}\n",
                    tar,
                    &record.now,
                    Paint::yellow(record.level).bold(),
                    record.module_path,
                    record.args,
                    record.format_line()
                );
            }
            _ => {
                data = format!(
                    "{} {} {} {} - {}\n",
                    tar, &record.now, Paint::blue(record.level).bold(), record.module_path, record.args
                );
            }
        }
        print!("{}", data);
        log_file.write(record.formated.as_bytes());
        log_file.flush();
        if log_file.metadata().unwrap().len() > self.max_size.get_len() as u64 {
            log_file.seek(SeekFrom::Start(0));
        }
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

struct ExplorerLogRecordFormat;

impl RecordFormat for ExplorerLogRecordFormat {
    fn do_format(&self, arg: &mut FastLogRecord) {
        let data;
        let now = format!("{:36}", arg.now.to_string());
        let tar = Paint::blue("Explorer API").bold();
        match arg.level {
            Level::Warn | Level::Error => {
                data = format!(
                    "{} {} {} {} - {}  {}\n",
                    tar,
                    &now,
                    arg.level,
                    arg.module_path,
                    arg.args,
                    arg.format_line()
                );
            }
            _ => {
                data = format!(
                    "{} {} {} {} - {}\n",
                    tar, &now, arg.level, arg.module_path, arg.args
                );
            }
        }
        arg.formated = data;
    }
}


impl ExplorerLog {
    pub fn init(settings: &Settings) -> Result<FastLogWaitGroup, LogError> {
        let log_dir_path = env::current_dir().unwrap()
            .join(&settings.log.log_dir);
        fs::create_dir_all(&log_dir_path).ok();

        let explorer_appender = ExplorerLogAppender::new(
            log_dir_path.join(&settings.log.file_name), file_size(&settings.log.max_size));

        let appenders: Vec<Box<dyn LogAppender>> = vec![Box::new(FileSplitAppender::new(
            log_dir(&settings.log.log_dir),
            file_size(&settings.log.temp_size),
            rolling_type(&settings.log.rolling_type),
            settings.log.zip_compress,
            1,
        )), Box::new(explorer_appender)];
        return init_custom_log(
            appenders,
            settings.log.log_cup,
            log_level(&settings.log.level),
            Box::new(NoFilter {}),
            Box::new(ExplorerLogRecordFormat),
        );
    }
}

fn log_dir(arg: &String) -> &'static str {
    if !arg.ends_with("/") {
        Box::leak(format!("{}/", arg).into_boxed_str())
    } else {
        Box::leak(format!("{}", arg).into_boxed_str())
    }
}

fn file_size(arg: &str) -> LogSize {
    match arg {
        arg if arg.ends_with("MB") => {
            let end = arg.find("MB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("KB") => {
            let end = arg.find("KB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        arg if arg.ends_with("GB") => {
            let end = arg.find("GB").unwrap();
            let num = arg[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn rolling_type(arg: &str) -> RollingType {
    match arg {
        arg if arg.starts_with("KeepNum(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        arg if arg.starts_with("KeepTime(") => {
            let end = arg.find(")").unwrap();
            let num = arg["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn log_level(arg: &str) -> llog::Level {
    return match arg {
        "error" => llog::Level::Error,
        "warn" => llog::Level::Warn,
        "trace" => llog::Level::Trace,
        "debug" => llog::Level::Debug,
        _ => llog::Level::Info,
    };
}


