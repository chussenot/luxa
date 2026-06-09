//! Colour names and parsing.

use crate::device::{LuxaforError, Rgb};

/// Built-in named colours, in display order.
pub const COLORS: &[(&str, Rgb)] = &[
    ("red", (255, 0, 0)),
    ("green", (0, 255, 0)),
    ("blue", (0, 0, 255)),
    ("yellow", (255, 255, 0)),
    ("cyan", (0, 255, 255)),
    ("magenta", (255, 0, 255)),
    ("white", (255, 255, 255)),
    ("orange", (255, 80, 0)),
    ("purple", (128, 0, 128)),
    ("pink", (255, 40, 90)),
    ("off", (0, 0, 0)),
];

/// Parse a colour name (`red`) or hex string (`#ff8800` / `ff8800`).
pub fn parse_color(value: &str) -> Result<Rgb, LuxaforError> {
    let key = value.trim().to_ascii_lowercase();
    if let Some((_, rgb)) = COLORS.iter().find(|(name, _)| *name == key) {
        return Ok(*rgb);
    }
    let hex = key.strip_prefix('#').unwrap_or(&key);
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Ok((r, g, b));
        }
    }
    let names = COLORS
        .iter()
        .map(|(n, _)| *n)
        .collect::<Vec<_>>()
        .join(", ");
    Err(LuxaforError(format!(
        "unknown colour {value:?}. Use a name ({names}) or hex like '#ff8800'."
    )))
}
