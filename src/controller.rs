//! Thread-safe access to the single Luxafor device for the server.
//!
//! Opens one handle and serializes behaviors behind a mutex (held by the
//! caller). Reconnects once if the device drops.

use crate::behaviors;
use crate::color;
use crate::config::WebhookSpec;
use crate::device::{Luxafor, LuxaforError};

#[derive(Default)]
pub struct Controller {
    dev: Option<Luxafor>,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    fn device(&mut self) -> Result<&mut Luxafor, LuxaforError> {
        if self.dev.is_none() {
            self.dev = Some(Luxafor::open()?);
        }
        Ok(self.dev.as_mut().expect("just set"))
    }

    /// Run the behavior named by `spec`. Reopens and retries once on error.
    pub fn run(&mut self, spec: &WebhookSpec) -> Result<(), LuxaforError> {
        let rgb = color::parse_color(&spec.color)?;
        if behaviors::run(self.device()?, rgb, spec).is_ok() {
            return Ok(());
        }
        // Handle may be stale (replug, sleep); reopen and retry once.
        self.dev = None;
        behaviors::run(self.device()?, rgb, spec)
    }
}
