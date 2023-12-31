// Copyright 2018 The Rio Advancement Inc
//

use std::fmt;
use std::io::{self, BufRead, BufReader, Read, Stdout, Write};
use std::env;
use uuid::Uuid;
use std::fs::{self, File};

use std::process::{self, Command};

use ansi_term::Colour;
use pbr;
use term::terminfo::TermInfo;
use term::{Terminal, TerminfoTerminal};
use serde::Serialize;
use serde_json;

use error::{Result, Error};
use self::tty::StdStream;

pub const NONINTERACTIVE_ENVVAR: &'static str = "RIO_NONINTERACTIVE";

pub const NOCOLORING_ENVVAR: &'static str = "RIO_NOCOLORING";

pub enum Status {
    Applying,
    Cached,
    Creating,
    Deleting,
    Determining,
    Downloading,
    Encrypting,
    Installed,
    Missing,
    Signing,
    Signed,
    Uploaded,
    Uploading,
    Using,
    Verified,
    Promoted,
    Custom(char, String),
}

impl Status {
    pub fn parts(&self) -> (char, String, Colour) {
        match *self {
            Status::Applying => ('↑', "Applying".into(), Colour::Green),
            Status::Cached => ('☑', "Cached".into(), Colour::Green),
            Status::Creating => ('Ω', "Creating".into(), Colour::Green),
            Status::Deleting => ('☒', "Deleting".into(), Colour::Green),
            Status::Determining => ('→', "Determining".into(), Colour::Green),
            Status::Downloading => ('↓', "Downloading".into(), Colour::Green),
            Status::Encrypting => ('☛', "Encrypting".into(), Colour::Green),
            Status::Installed => ('✓', "Installed".into(), Colour::Green),
            Status::Missing => ('∵', "Missing".into(), Colour::Red),
            Status::Signed => ('✓', "Signed".into(), Colour::Cyan),
            Status::Signing => ('☛', "Signing".into(), Colour::Cyan),
            Status::Uploaded => ('✓', "Uploaded".into(), Colour::Green),
            Status::Uploading => ('↑', "Uploading".into(), Colour::Green),
            Status::Using => ('→', "Using".into(), Colour::Green),
            Status::Verified => ('✓', "Verified".into(), Colour::Green),
            Status::Promoted => ('✓', "Promoted".into(), Colour::Green),
            Status::Custom(c, ref s) => (c, s.to_string(), Colour::Green),
        }
    }
}

pub struct UI {
    shell: Shell,
}

impl UI {
    pub fn default_with(coloring: Coloring, isatty: Option<bool>) -> Self {
        UI { shell: Shell::default_with(coloring, isatty) }
    }

    pub fn begin<T: ToString>(&mut self, message: T) -> Result<()> {
        Self::write_heading(&mut self.shell.out, Colour::Yellow, '»', message)
    }

    pub fn end<T: ToString>(&mut self, message: T) -> Result<()> {
        Self::write_heading(&mut self.shell.out, Colour::Blue, '★', message)
    }

    pub fn is_a_tty(&self) -> bool {
        self.shell.input.isatty && self.shell.out.isatty && self.shell.err.isatty
    }

    pub fn status<T: fmt::Display>(&mut self, status: Status, message: T) -> Result<()> {
        let ref mut stream = self.shell.out;
        let (symbol, status_str, color) = status.parts();
        match stream.is_colored() {
            true => {
                try!(write!(
                    stream,
                    "{} {}\n",
                    color.bold().paint(format!("{} {}", symbol, status_str)),
                    message.to_string()
                ))
            }
            false => {
                try!(write!(
                    stream,
                    "{} {} {}\n",
                    symbol,
                    status_str,
                    message.to_string()
                ))
            }
        }
        try!(stream.flush());
        Ok(())
    }

    pub fn warn<T: fmt::Display>(&mut self, message: T) -> Result<()> {
        let ref mut stream = self.shell.err;
        match stream.is_colored() {
            true => {
                try!(write!(
                    stream,
                    "{}\n",
                    Colour::Yellow.bold().paint(
                        format!("∅ {}", message.to_string()),
                    )
                ));
            }
            false => {
                try!(write!(stream, "∅ {}\n", message.to_string()));
            }
        }
        try!(stream.flush());
        Ok(())
    }

    pub fn fatal<T: fmt::Display>(&mut self, message: T) -> Result<()> {
        let ref mut stream = self.shell.err;
        let formatted_message = message
            .to_string()
            .lines()
            .map(|line| format!("✗✗✗ {}", line))
            .collect::<Vec<_>>()
            .join("\n");

        match stream.is_colored() {
            true => {
                try!(write!(
                    stream,
                    "{}\n",
                    Colour::Red.bold().paint(format!(
                        "✗✗✗\n{}\n✗✗✗",
                        formatted_message
                    ))
                ));
            }
            false => {
                try!(write!(
                    stream,
                    "✗✗✗\n{}\n✗✗✗\n",
                    formatted_message
                ));
            }
        }
        try!(stream.flush());
        Ok(())
    }

    pub fn progress(&mut self) -> Option<ProgressBar> {
        if self.shell.out.is_a_terminal() {
            Some(ProgressBar::default())
        } else {
            None
        }
    }

    pub fn title(&mut self, text: &str) -> Result<()> {
        let ref mut stream = self.shell.out;
        match stream.is_colored() {
            true => {
                try!(write!(stream, "{}\n", Colour::Green.bold().paint(text)));
                try!(write!(
                    stream,
                    "{}\n\n",
                    Colour::Green.bold().paint(format!(
                        "{:=<width$}",
                        "",
                        width = text.chars().count()
                    ))
                ));
            }
            false => {
                try!(write!(stream, "{}\n", text));
                try!(write!(
                    stream,
                    "{}\n\n",
                    format!("{:=<width$}", "", width = text.chars().count())
                ));
            }
        }
        try!(stream.flush());
        Ok(())
    }

    pub fn heading(&mut self, text: &str) -> Result<()> {
        let ref mut stream = self.shell.out;
        match stream.is_colored() {
            true => {
                try!(write!(stream, "{}\n\n", Colour::Green.bold().paint(text)));
            }
            false => {
                try!(write!(stream, "{}\n\n", text));
            }
        }
        try!(stream.flush());
        Ok(())
    }

    pub fn para(&mut self, text: &str) -> Result<()> {
        Self::print_wrapped(&mut self.shell.out, text, 75, 2)
    }

    pub fn br(&mut self) -> Result<()> {
        let ref mut stream = self.shell.out;
        try!(write!(stream, "\n"));
        try!(stream.flush());
        Ok(())
    }

    pub fn prompt_yes_no(&mut self, question: &str, default: Option<bool>) -> Result<bool> {
        let ref mut stream = self.shell.out;
        let choice = match default {
            Some(yes) => {
                if yes {
                    match stream.is_colored() {
                        true => {
                            format!(
                                "{}{}{}",
                                Colour::White.paint("["),
                                Colour::White.bold().paint("Yes"),
                                Colour::White.paint("/no/quit]")
                            )
                        }
                        false => format!("[Yes/no/quit]"),
                    }
                } else {
                    match stream.is_colored() {
                        true => {
                            format!(
                                "{}{}{}",
                                Colour::White.paint("[yes/"),
                                Colour::White.bold().paint("No"),
                                Colour::White.paint("/quit]")
                            )
                        }
                        false => format!("[yes/No/quit]"),
                    }
                }
            }
            None => {
                match stream.is_colored() {
                    true => format!("{}", Colour::White.paint("[yes/no/quit]")),
                    false => format!("[yes/no/quit]"),
                }
            }
        };
        loop {
            try!(stream.flush());
            match stream.is_colored() {
                true => {
                    try!(write!(
                        stream,
                        "{} {} ",
                        Colour::Cyan.paint(question),
                        choice
                    ));
                }
                false => {
                    try!(write!(stream, "{} {} ", question, choice));
                }
            }
            try!(stream.flush());
            let mut response = String::new();
            {
                let reference = self.shell.input.by_ref();
                try!(BufReader::new(reference).read_line(&mut response));
            }
            match response.trim().chars().next().unwrap_or('\n') {
                'y' | 'Y' => return Ok(true),
                'n' | 'N' => return Ok(false),
                'q' | 'Q' => process::exit(0),
                '\n' => {
                    match default {
                        Some(default) => return Ok(default),
                        None => continue,
                    }
                }
                _ => continue,
            }
        }
    }

    pub fn prompt_ask(&mut self, question: &str, default: Option<&str>) -> Result<String> {
        let ref mut stream = self.shell.out;
        let choice = match default {
            Some(d) => {
                match stream.is_colored() {
                    true => {
                        format!(
                            " {}{}{}",
                            Colour::White.paint("[default: "),
                            Colour::White.bold().paint(d),
                            Colour::White.paint("]")
                        )
                    }
                    false => format!(" [default: {}]", d),
                }
            }
            None => "".to_string(),
        };
        loop {
            try!(stream.flush());
            match stream.is_colored() {
                true => {
                    try!(write!(
                        stream,
                        "{}{} ",
                        Colour::Cyan.paint(format!("{}:", question)),
                        choice
                    ));
                }
                false => {
                    try!(write!(stream, "{}{} ", format!("{}:", question), choice));
                }
            }
            try!(stream.flush());
            let mut response = String::new();
            {
                let reference = self.shell.input.by_ref();
                try!(BufReader::new(reference).read_line(&mut response));
            }
            if response.trim().is_empty() {
                match default {
                    Some(d) => return Ok(d.to_string()),
                    None => continue,
                }
            }
            return Ok(response.trim().to_string());
        }
    }


    pub fn edit<T: Serialize>(&mut self, contents: &T) -> Result<String> {
        let encoded = serde_json::to_string_pretty(contents).unwrap();
        let editor = env::var("EDITOR").map_err(|e| Error::EditorEnv(e))?;
        let mut tmp_file_path = env::temp_dir();
        tmp_file_path.push(format!("_rioos_{}.tmp", Uuid::new_v4()));

        let mut tmp_file = try!(File::create(&tmp_file_path));
        tmp_file.write_all(encoded.as_bytes())?;


        tmp_file.sync_all()?;

        let mut cmd = Command::new(editor);
        cmd.arg(tmp_file_path.display().to_string());

        let status = cmd.spawn()?.wait()?;

        if !status.success() {
            debug!("Failed edit with status: {:?}", status);
            return Err(Error::EditStatus);
        }

        let mut out = String::new();
        tmp_file = File::open(&tmp_file_path)?;
        tmp_file.read_to_string(&mut out)?;

        fs::remove_file(tmp_file_path)?;
        Ok(out)
    }

    fn write_heading<T: ToString>(stream: &mut OutputStream, color: Colour, symbol: char, message: T) -> Result<()> {
        match stream.is_colored() {
            true => {
                try!(write!(
                    stream,
                    "{}\n",
                    color.bold().paint(
                        format!("{} {}", symbol, message.to_string()),
                    )
                ))
            }
            false => try!(write!(stream, "{} {}\n", symbol, message.to_string())),
        }
        try!(stream.flush());
        Ok(())
    }

    fn print_wrapped(stream: &mut OutputStream, text: &str, wrap_width: usize, left_indent: usize) -> Result<()> {
        for line in text.split("\n\n") {
            let mut buffer = String::new();
            let mut width = 0;
            for word in line.split_whitespace() {
                let wl = word.chars().count();
                if (width + wl + 1) > (wrap_width - left_indent) {
                    try!(write!(
                        stream,
                        "{:<width$}{}\n",
                        " ",
                        buffer,
                        width = left_indent
                    ));
                    buffer.clear();
                    width = 0;
                }
                width = width + wl + 1;
                buffer.push_str(word);
                buffer.push(' ');
            }
            if !buffer.is_empty() {
                try!(write!(
                    stream,
                    "{:<width$}{}\n",
                    " ",
                    buffer,
                    width = left_indent
                ));
            }
            try!(write!(stream, "\n"));
        }
        try!(stream.flush());
        Ok(())
    }
}

impl Default for UI {
    fn default() -> Self {
        UI::default_with(Coloring::Auto, None)
    }
}

pub struct Shell {
    input: InputStream,
    out: OutputStream,
    err: OutputStream,
}

impl Shell {
    pub fn new(input: InputStream, out: OutputStream, err: OutputStream) -> Self {
        Shell {
            input: input,
            out: out,
            err: err,
        }
    }

    pub fn default_with(coloring: Coloring, isatty: Option<bool>) -> Self {
        let stdin = InputStream::from_stdin(isatty);
        debug!(
            "InputStream(stdin): {{ is_a_terminal(): {} }}",
            stdin.is_a_terminal()
        );
        let stdout = OutputStream::from_stdout(coloring, isatty);
        debug!(
            "OutputStream(stdout): {{ is_colored(): {}, supports_color(): {}, \
             is_a_terminal(): {} }}",
            stdout.is_colored(),
            stdout.supports_color(),
            stdout.is_a_terminal()
        );
        let stderr = OutputStream::from_stderr(coloring, isatty);
        debug!(
            "OutputStream(stderr): {{ is_colored(): {}, supports_color(): {}, \
             is_a_terminal(): {} }}",
            stderr.is_colored(),
            stderr.supports_color(),
            stderr.is_a_terminal()
        );
        Shell::new(stdin, stdout, stderr)
    }

    pub fn input(&mut self) -> &mut InputStream {
        &mut self.input
    }

    pub fn out(&mut self) -> &mut OutputStream {
        &mut self.out
    }

    pub fn err(&mut self) -> &mut OutputStream {
        &mut self.err
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell::default_with(Coloring::Auto, None)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Coloring {
    Auto,
    Always,
    Never,
}

pub struct InputStream {
    inner: Box<Read + Send>,
    isatty: bool,
}

impl InputStream {
    pub fn new(inner: Box<Read + Send>, isatty: bool) -> Self {
        InputStream {
            inner: inner,
            isatty: isatty,
        }
    }

    pub fn from_stdin(isatty: Option<bool>) -> Self {
        Self::new(
            Box::new(io::stdin()),
            match isatty {
                Some(val) => val,
                None => tty::isatty(StdStream::Stdin),
            },
        )
    }

    pub fn is_a_terminal(&self) -> bool {
        self.isatty
    }
}

impl Read for InputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

pub struct OutputStream {
    inner: WriteStream,
    coloring: Coloring,
    isatty: bool,
}

impl OutputStream {
    pub fn new(inner: WriteStream, coloring: Coloring, isatty: bool) -> Self {
        OutputStream {
            inner: inner,
            coloring: coloring,
            isatty: isatty,
        }
    }

    pub fn from_stdout(coloring: Coloring, isatty: Option<bool>) -> Self {
        Self::new(
            WriteStream::create(|| Box::new(io::stdout())),
            coloring,
            match isatty {
                Some(val) => val,
                None => tty::isatty(StdStream::Stdout),
            },
        )
    }

    pub fn from_stderr(coloring: Coloring, isatty: Option<bool>) -> Self {
        Self::new(
            WriteStream::create(|| Box::new(io::stderr())),
            coloring,
            match isatty {
                Some(val) => val,
                None => tty::isatty(StdStream::Stderr),
            },
        )
    }

    pub fn supports_color(&self) -> bool {
        match self.inner {
            WriteStream::Color(_) => true,
            WriteStream::NoColor(_) => false,
        }
    }

    pub fn is_colored(&self) -> bool {
        self.supports_color() && (Coloring::Auto == self.coloring || Coloring::Always == self.coloring)
    }

    pub fn is_a_terminal(&self) -> bool {
        self.isatty
    }
}

impl Write for OutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.inner {
            WriteStream::Color(ref mut io) => io.write(buf),
            WriteStream::NoColor(ref mut io) => io.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.inner {
            WriteStream::Color(ref mut io) => io.flush(),
            WriteStream::NoColor(ref mut io) => io.flush(),
        }
    }
}
pub enum WriteStream {
    NoColor(Box<Write + Send>),
    Color(Box<Terminal<Output = Box<Write + Send>> + Send>),
}

impl WriteStream {
    // Implementation heavily inspired and based on the Cargo `shell.rs` implementation. Source:
    // https://github.com/rust-lang/cargo/blob/d05ba53afec82308edcfeb778446010bf18e71ae/
    // src/cargo/core/shell.rs

    pub fn create<T: FnMut() -> Box<Write + Send>>(mut writable_fn: T) -> Self {
        match Self::get_term(writable_fn()) {
            Ok(t) => t,
            Err(_) => WriteStream::NoColor(writable_fn()),
        }
    }

    #[cfg(any(windows))]
    fn get_term(writeable: Box<Write + Send>) -> Result<Self> {
        // Check if the creation of a console will succeed
        if ::term::WinConsole::new(vec![0u8; 0]).is_ok() {
            let term = try!(::term::WinConsole::new(writeable));
            if !term.supports_color() {
                Ok(WriteStream::NoColor(Box::new(term)))
            } else {
                Ok(WriteStream::Color(Box::new(term)))
            }
        } else {
            // If we fail to get a windows console, we try to get a `TermInfo` one
            Ok(Self::get_terminfo_term(writeable))
        }
    }

    #[cfg(any(unix))]
    fn get_term(writeable: Box<Write + Send>) -> Result<Self> {
        Ok(Self::get_terminfo_term(writeable))
    }

    fn get_terminfo_term(writeable: Box<Write + Send>) -> Self {
        // Use `TermInfo::from_env()` and `TerminfoTerminal::supports_color()` to determine if
        // creation of a TerminfoTerminal is possible regardless of the tty status. --color options
        // are parsed after Shell creation so always try to create a terminal that supports color
        // output. Fall back to a no-color terminal regardless of whether or not a tty is present
        // and if color output is not possible.
        match TermInfo::from_env() {
            Ok(info) => {
                let term = TerminfoTerminal::new_with_terminfo(writeable, info);
                if !term.supports_color() {
                    WriteStream::NoColor(term.into_inner())
                } else {
                    WriteStream::Color(Box::new(term))
                }
            }
            Err(_) => WriteStream::NoColor(writeable),
        }
    }
}

mod tty {
    pub enum StdStream {
        Stdin,
        Stdout,
        Stderr,
    }

    #[cfg(unix)]
    pub fn isatty(output: StdStream) -> bool {
        extern crate libc;

        let fd = match output {
            StdStream::Stdin => libc::STDIN_FILENO,
            StdStream::Stdout => libc::STDOUT_FILENO,
            StdStream::Stderr => libc::STDERR_FILENO,
        };

        unsafe { libc::isatty(fd) != 0 }
    }
    #[cfg(windows)]
    pub fn isatty(output: StdStream) -> bool {
        extern crate kernel32;
        extern crate winapi;

        let handle = match output {
            StdStream::Stdin => winapi::winbase::STD_INPUT_HANDLE,
            StdStream::Stdout => winapi::winbase::STD_OUTPUT_HANDLE,
            StdStream::Stderr => winapi::winbase::STD_ERROR_HANDLE,
        };

        unsafe {
            let handle = kernel32::GetStdHandle(handle);
            let mut out = 0;
            kernel32::GetConsoleMode(handle, &mut out) != 0
        }
    }
}

/// A moving progress bar to track progress of a sized event, similar to wget, curl, npm, etc.
///
/// This is designed to satisfy a generic behavior which sets the size of the task (usually a
/// number of bytes representing the total download/upload/transfer size) and will be a generic
/// writer (i.e. implementing the `Write` trait) as a means to increase progress towards
/// completion.
pub struct ProgressBar {
    bar: pbr::ProgressBar<Stdout>,
    total: u64,
    current: u64,
}

impl Default for ProgressBar {
    fn default() -> Self {
        ProgressBar {
            bar: pbr::ProgressBar::new(0),
            total: 0,
            current: 0,
        }
    }
}

pub trait DisplayProgress: Write {
    fn size(&mut self, size: u64);
    fn finish(&mut self);
}

impl DisplayProgress for ProgressBar {
    fn size(&mut self, size: u64) {
        self.bar = pbr::ProgressBar::new(size);
        self.bar.set_units(pbr::Units::Bytes);
        self.bar.show_tick = true;
        self.bar.message("    ");
        self.total = size;
    }

    fn finish(&mut self) {
        println!("");
        io::stdout().flush().ok().expect("flush() fail");
    }
}

impl Write for ProgressBar {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.bar.write(buf) {
            Ok(n) => {
                self.current += n as u64;
                if self.current == self.total {
                    self.finish();
                }
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.bar.flush()
    }
}

/// Ideally wanted to use the commons::ui::UI, but take a hack  here by directly assuming we have a colored shell.
/// this is not good though.
/// May be we should pass the UI into the Handler code.
pub fn rawdump<T: ToString>(color: Colour, symbol: char, message: T) {
    print!(
        "{}",
        color.paint(format!("{} {}", symbol, message.to_string()))
    );
}

pub fn rawdumpln<T: ToString>(color: Colour, symbol: char, message: T) {
    println!(
        "{}",
        color.paint(format!("{} {}", symbol, message.to_string()))
    );
}
