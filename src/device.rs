//! Low-level driver for the Luxafor Flag over USB (pure-Rust `nusb`, no libusb).
//!
//! The Flag (VID 0x04d8 / PID 0xf372) is a HID device with a single 8-byte
//! interrupt OUT endpoint (0x01). We detach the kernel HID driver, claim the
//! interface, and send 8-byte command packets directly:
//!
//! ```text
//! [mode, arg, R, G, B, arg, arg, arg]
//! ```
//!
//! Modes:  1 static · 2 fade · 3 strobe(flash) · 4 wave

use std::fmt;
use std::thread::sleep;
use std::time::Duration;

use nusb::transfer::{Buffer, Interrupt, Out};
use nusb::{Device, Endpoint, Interface, MaybeFuture};

pub const VENDOR_ID: u16 = 0x04d8;
pub const PRODUCT_ID: u16 = 0xf372;
const EP_OUT: u8 = 0x01;
const LED_ALL: u8 = 0xff;

const MODE_STATIC: u8 = 1;
const MODE_FADE: u8 = 2;
const MODE_STROBE: u8 = 3;
const MODE_WAVE: u8 = 4;

/// An (R, G, B) colour.
pub type Rgb = (u8, u8, u8);

#[derive(Debug)]
pub struct LuxaforError(pub String);

impl fmt::Display for LuxaforError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for LuxaforError {}

/// A connected Luxafor Flag.
pub struct Luxafor {
    ep: Endpoint<Interrupt, Out>,
    // Held so the interface/device stay open for the endpoint's lifetime.
    _interface: Interface,
    _device: Device,
}

impl Luxafor {
    /// Find and open the first Luxafor Flag.
    pub fn open() -> Result<Self, LuxaforError> {
        let info = nusb::list_devices()
            .wait()
            .map_err(|e| LuxaforError(format!("could not list USB devices: {e}")))?
            .find(|d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID)
            .ok_or_else(|| LuxaforError("Luxafor not found — is it plugged in?".to_string()))?;

        let device = info.open().wait().map_err(|e| {
            LuxaforError(format!(
                "could not open the Luxafor: {e}. On Linux you may need the udev \
                 rule (see README)."
            ))
        })?;

        let interface = device
            .detach_and_claim_interface(0)
            .wait()
            .map_err(|e| LuxaforError(format!("could not claim USB interface: {e}")))?;

        let ep = interface
            .endpoint::<Interrupt, Out>(EP_OUT)
            .map_err(|e| LuxaforError(format!("could not open the OUT endpoint: {e}")))?;

        Ok(Self {
            ep,
            _interface: interface,
            _device: device,
        })
    }

    /// Send one 8-byte command packet and wait for it to complete.
    fn write(&mut self, payload: [u8; 8]) -> Result<(), LuxaforError> {
        self.ep.submit(Buffer::from(payload.to_vec()));
        let completion = self
            .ep
            .wait_next_complete(Duration::from_secs(2))
            .ok_or_else(|| LuxaforError("USB write timed out".to_string()))?;
        completion
            .status
            .map_err(|e| LuxaforError(format!("USB write failed: {e:?}")))
    }

    /// Set a solid colour immediately.
    pub fn set_color(&mut self, (r, g, b): Rgb) -> Result<(), LuxaforError> {
        self.write([MODE_STATIC, LED_ALL, r, g, b, 0, 0, 0])
    }

    /// Turn all LEDs off.
    pub fn off(&mut self) -> Result<(), LuxaforError> {
        self.set_color((0, 0, 0))
    }

    /// Fade to a colour. `duration` is in device ticks (~10ms).
    pub fn fade(&mut self, (r, g, b): Rgb, duration: u8) -> Result<(), LuxaforError> {
        self.write([MODE_FADE, LED_ALL, r, g, b, duration, 0, 0])
    }

    /// Hardware strobe/flash. `speed` and `repeat` are 0-255.
    pub fn strobe(&mut self, (r, g, b): Rgb, speed: u8, repeat: u8) -> Result<(), LuxaforError> {
        self.write([MODE_STROBE, LED_ALL, r, g, b, speed, 0, repeat])
    }

    /// Hardware wave effect. `wave_type` 1-5, `repeat`/`speed` 0-255.
    pub fn wave(
        &mut self,
        (r, g, b): Rgb,
        wave_type: u8,
        repeat: u8,
        speed: u8,
    ) -> Result<(), LuxaforError> {
        self.write([MODE_WAVE, wave_type, r, g, b, 0, repeat, speed])
    }

    /// Software blink: toggle the colour on/off `times` times.
    pub fn blink(
        &mut self,
        rgb: Rgb,
        times: u32,
        on: Duration,
        off: Duration,
    ) -> Result<(), LuxaforError> {
        for _ in 0..times {
            self.set_color(rgb)?;
            sleep(on);
            self.set_color((0, 0, 0))?;
            sleep(off);
        }
        Ok(())
    }
}
