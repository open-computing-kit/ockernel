use core::{fmt, fmt::Write};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

use x86::io::{inb, outb};

/// Write a string to the output channel
///
/// # Safety
/// This method is unsafe because it does port accesses without synchronisation
pub unsafe fn puts(s: &str) {
    for b in s.bytes() {
        putb(b);
    }
}

/// Write a single byte to the output channel
///
/// # Safety
/// This method is unsafe because it does port accesses without synchronisation
pub unsafe fn putb(b: u8) {
    // Wait for the serial port's fifo to not be empty
    while (inb(0x3F8 + 5) & 0x20) == 0 {
        // Do nothing
    }
    // Send the byte out the serial port
    outb(0x3F8, b);

    // Also send to the bochs 0xe9 hack
    outb(0xe9, b);
}

/// wrapper struct to allow us to "safely" write!() to the serial port
///
/// we don't worry about synchronization and locking since that creates more problems than it's worth for a simple debugging interface
struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            puts(s);
        }
        Ok(())
    }
}

/// simple logger implementation over serial
struct Logger {
    pub max_level: LevelFilter,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    #[allow(unused_must_use)]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(path) = record.module_path() {
                writeln!(&mut SerialWriter, "{:width$} [{}] {}", record.level(), path, record.args(), width = 5);
            } else {
                writeln!(&mut SerialWriter, "{:width$} [unknown] {}", record.level(), record.args(), width = 5);
            }
        }
    }

    fn flush(&self) {}
}

/// our logger that we will log things with
static LOGGER: Logger = Logger { max_level: LevelFilter::Info };

/// initialize the logger, setting the max level in the process
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LOGGER.max_level))
}
