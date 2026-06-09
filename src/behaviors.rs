//! Named behaviors mapping a webhook to a device action.
//!
//! Single source of truth for which behaviors exist: [`config`](crate::config)
//! validates against [`is_known`] and the CLI lists [`BEHAVIORS`].

use std::time::Duration;

use crate::config::WebhookSpec;
use crate::device::{Luxafor, LuxaforError, Rgb};

/// `(name, description)` for every behavior, in display order.
pub const BEHAVIORS: &[(&str, &str)] = &[
    ("solid", "Set a steady colour"),
    ("off", "Turn the light off"),
    ("blink", "Software blink, ~0.3s on/off"),
    ("slow_blink", "Slow software blink, ~0.8s on/off"),
    ("fast_blink", "Fast software blink, ~0.1s on/off"),
    ("flash", "Hardware strobe/flash"),
    ("fade", "Fade to a colour"),
    ("pulse", "Hardware wave/pulse effect"),
];

/// Is `name` a known behavior?
pub fn is_known(name: &str) -> bool {
    BEHAVIORS.iter().any(|(n, _)| *n == name)
}

fn secs(value: Option<f64>, default: f64) -> Duration {
    Duration::from_secs_f64(value.unwrap_or(default))
}

/// Run the behavior named by `spec.behavior` on `lux`.
pub fn run(lux: &mut Luxafor, rgb: Rgb, spec: &WebhookSpec) -> Result<(), LuxaforError> {
    match spec.behavior.as_str() {
        "solid" => lux.set_color(rgb),
        "off" => lux.off(),
        "blink" => blink(
            lux,
            rgb,
            spec.times.unwrap_or(5),
            secs(spec.on, 0.3),
            secs(spec.off, 0.3),
        ),
        "slow_blink" => blink(
            lux,
            rgb,
            spec.times.unwrap_or(3),
            secs(spec.on, 0.8),
            secs(spec.off, 0.8),
        ),
        "fast_blink" => blink(
            lux,
            rgb,
            spec.times.unwrap_or(8),
            secs(spec.on, 0.1),
            secs(spec.off, 0.1),
        ),
        "flash" => lux.strobe(rgb, spec.speed.unwrap_or(10), spec.repeat.unwrap_or(5)),
        "fade" => lux.fade(rgb, spec.duration.unwrap_or(30)),
        "pulse" => lux.wave(
            rgb,
            spec.wave_type.unwrap_or(1),
            spec.repeat.unwrap_or(3),
            spec.speed.unwrap_or(30),
        ),
        other => Err(LuxaforError(format!("unknown behavior {other:?}"))),
    }
}

fn blink(
    lux: &mut Luxafor,
    rgb: Rgb,
    times: u32,
    on: Duration,
    off: Duration,
) -> Result<(), LuxaforError> {
    lux.blink(rgb, times, on, off)?;
    lux.off()
}
