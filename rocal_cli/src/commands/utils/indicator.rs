use core::time;
use std::{
    io::Write,
    sync::{Arc, Mutex},
    thread,
};

use super::color::Color;

///
/// Usage
///
/// ```rust
/// let mut indicator = IndicatorLauncher::new()
///     .kind(Kind::Spinner)
///     .interval(100)
///     .text("Processing...")
///     .color(Color::Blue)
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
}

pub struct Indicator {
    is_processing: Arc<Mutex<bool>>,
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
        let is_processing = Arc::new(Mutex::new(true));
        let is_processing_cloned = Arc::clone(&is_processing);
        let text = text.to_string();

        thread::spawn(move || {
            let mut f = std::io::stdout();
            let interval = time::Duration::from_millis(interval_millis);

            while {
                let is_processing = is_processing_cloned.lock().unwrap();
                *is_processing
            } {
                for i in kind.symbols().iter() {
                    write!(f, "\r{}{} {}{}", color.code(), i, text, Color::reset()).unwrap();
                    f.flush().unwrap();
                    thread::sleep(interval);
                }
            }
        });

        Self { is_processing }
    }

    pub fn stop(&mut self) -> Result<(), std::io::Error> {
        let mut is_processing = self.is_processing.lock().unwrap();
        *is_processing = false;

        let mut f = std::io::stdout();
        write!(f, "\r\x1b[2K")?;

        Ok(())
    }
}

impl Kind {
    fn symbols(&self) -> Vec<&str> {
        match self {
            Self::Spinner => vec!["|", "/", "-", "\\"],
        }
    }
}
