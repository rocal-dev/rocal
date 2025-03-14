use core::time;
use std::{
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use super::color::Color;

///
/// Usage
///
/// ```rust
/// let mut indicator = IndicatorLauncher::new()
///     .kind(Kind::Dots)
///     .interval(100)
///     .text("Processing...")
///     .color(Color::White)
///     .start();
///
/// thread::sleep(Duration::from_millis(1000));
///
/// indicator.stop()?;
///
/// let mut f = std::io::stdout();
///
/// writeln!(f, "{}", Color::Green.text("Done"))?;
/// f.flush()?;
/// ```

#[derive(Clone, Copy)]
pub enum Kind {
    Spinner,
    Dots,
}

pub struct Indicator {
    is_processing: Arc<AtomicBool>,
}

pub struct IndicatorLauncher {
    kind: Option<Kind>,
    interval_millis: Option<u64>,
    text: Option<String>,
    color: Option<Color>,
}

impl IndicatorLauncher {
    pub fn new() -> Self {
        Self {
            kind: None,
            interval_millis: None,
            text: None,
            color: None,
        }
    }

    pub fn kind(&mut self, kind: Kind) -> &mut Self {
        self.kind = Some(kind);
        self
    }

    pub fn interval(&mut self, millis: u64) -> &mut Self {
        self.interval_millis = Some(millis);
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    pub fn start(&mut self) -> Indicator {
        let kind = if let Some(kind) = self.kind {
            kind
        } else {
            Kind::Spinner
        };

        let interval = if let Some(interval) = self.interval_millis {
            interval
        } else {
            100
        };

        let text = if let Some(text) = &self.text {
            text
        } else {
            ""
        };

        let color = if let Some(color) = self.color {
            color
        } else {
            Color::White
        };

        Indicator::start(kind, interval, text, color)
    }
}

impl Indicator {
    pub fn start(kind: Kind, interval_millis: u64, text: &str, color: Color) -> Self {
        let is_processing = Arc::new(AtomicBool::new(true));
        let is_processing_cloned = Arc::clone(&is_processing);
        let text = text.to_string();
        let interval = time::Duration::from_millis(interval_millis);
        let check_interval = Duration::from_millis(10);

        thread::spawn(move || {
            let mut f = std::io::stdout();

            while is_processing_cloned.load(Ordering::SeqCst) {
                for i in kind.symbols().iter() {
                    if !is_processing_cloned.load(Ordering::SeqCst) {
                        break;
                    }

                    write!(f, "\r{}{} {}{}", color.code(), i, text, Color::reset()).unwrap();
                    f.flush().unwrap();

                    let mut elapsed = Duration::from_millis(0);
                    while elapsed < interval {
                        if !is_processing_cloned.load(Ordering::SeqCst) {
                            break;
                        }
                        thread::sleep(check_interval);
                        elapsed += check_interval;
                    }
                }
            }
        });

        Self { is_processing }
    }

    pub fn stop(&mut self) -> Result<(), std::io::Error> {
        self.is_processing.store(false, Ordering::SeqCst);

        let mut f = std::io::stdout();
        write!(f, "\r\x1b[2K")?;
        f.flush()?;

        Ok(())
    }
}

impl Kind {
    fn symbols(&self) -> Vec<&str> {
        match self {
            Self::Spinner => vec!["|", "/", "-", "\\"],
            Self::Dots => vec!["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
        }
    }
}
