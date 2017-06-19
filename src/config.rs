use std;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Dev,
    Prod,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Mode::Dev => write!(f, "dev"),
            Mode::Prod => write!(f, "prod"),
        }
    }
}

impl Mode {
    /// Choose mode from string.
    pub fn from_str(s: &str) -> Result<Mode, String> {
        use std::ascii::AsciiExt;

        match s.to_ascii_lowercase().as_str() {
            "dev" => Ok(Mode::Dev),
            "prod" => Ok(Mode::Prod),
            _ => Err(format!("cannot convert to mode: {}", s))
        }
    }
}

/// Zircon server configuration
#[derive(Clone)]
pub struct ZirconConfig {
    mode: Mode,
    respect_xforwarded: bool,
    num_accept_threads: usize,
    num_cpu_threads: usize,
}

impl ZirconConfig {
    /// The default development configuration.
    pub fn dev() -> ZirconConfig {
        Self::from_mode(Mode::Dev)
    }

    /// The default production configuration.
    pub fn prod() -> ZirconConfig {
        Self::from_mode(Mode::Prod)
    }

    /// Choose configuration from mode.
    pub fn from_mode(mode: Mode) -> ZirconConfig {
        // TODO(mayah): This should be based on the current number of cpus?
        let num_accept_threads = match mode {
            Mode::Dev => 1,
            Mode::Prod => 4,
        };
        let num_cpu_threads = match mode {
            Mode::Dev => 4,
            Mode::Prod => 16,
        };

        ZirconConfig {
            mode: mode,
            respect_xforwarded: false,
            num_accept_threads: num_accept_threads,
            num_cpu_threads: num_cpu_threads,
        }
    }

    /// When `respect_xforwarded` is true, Request will respect X-Forwarded-*.
    /// host(), port(), scheme(), and remote_addr() will respect X-Forwarded-*.
    /// It will be useful when an application is running behind a reverse proxy.
    pub fn with_respect_xforwarded(mut self, b: bool) -> ZirconConfig {
        self.respect_xforwarded = b;
        self
    }

    pub fn with_num_accept_threads(mut self, n: usize) -> ZirconConfig {
        self.num_accept_threads = n;
        self
    }

    pub fn with_num_cpu_threads(mut self, n: usize) -> ZirconConfig {
        self.num_cpu_threads = n;
        self
    }

    /// Server mode of the current configuration
    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn num_accept_threads(&self) -> usize {
        self.num_accept_threads
    }

    pub fn num_cpu_threads(&self) -> usize {
        self.num_cpu_threads
    }
}
