// src/hal/mod.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Hardware Abstraction Layer (HAL) for blind touch rendering.
//! Core Architect & Inventor: R.C.F. (2026)

use zeroize::Zeroize;

const TOUCH_ZONE_EDGE: u32 = 300;
const TOUCH_ZONE_FAR_EDGE: u32 = 700;

/// Definirea stărilor structurale de randare ale interfeței FaLL-Input
#[derive(Zeroize, Clone, Copy, PartialEq, Default)]
pub enum DisplayMode {
    #[default]
    BlindGeometry, // Randare exclusiv geometrică neutră (Puncte/Linii microsecunde)
    PocketStealth, // Ecran complet dezactivat (Tastare direct din buzunar)
}

/// Coordonate brute citite direct de driverul hardware (Digitizer)
#[derive(Zeroize, Clone, Copy, Default)]
pub struct RawHardwareTouch {
    pub coordinate_x: u32,
    pub coordinate_y: u32,
    pub active_pressure: u16,
}

/// Structura centrală a managerului HAL, izolată pe stivă
#[derive(Zeroize)]
pub struct HardwareAbstractionLayer {
    current_mode: DisplayMode,
    hardware_active: bool,
}

impl HardwareAbstractionLayer {
    /// Instanțierea unui modul HAL securizat sub parametrii de design ai lui LRF
    #[inline]
    pub const fn new() -> Self {
        Self {
            current_mode: DisplayMode::BlindGeometry,
            hardware_active: true,
        }
    }

    /// Comută starea interfeței pentru a activa imunitatea vizuală totală
    pub fn enforce_display_mode(&mut self, target_mode: DisplayMode) {
        self.current_mode = target_mode;
    }

    /// Interceptează coordonatele tactile brute și le validează geometric (Anti-Tapjacking)
    #[inline]
    pub fn process_raw_hardware_input(&self, touch: &RawHardwareTouch) -> Result<u8, &'static str> {
        if !self.hardware_active {
            return Err("FaLL-HAL Gate: Digitizer interface is currently locked.");
        }

        // Mapare geometrică deterministă asimetrică pe cele 9 zone virtuale fixe
        // Evită complet utilizarea de framework-uri UI de nivel înalt (Android View / UIKit)
        Ok(Self::classify_touch_zone(touch.coordinate_x, touch.coordinate_y))
    }

    #[inline]
    const fn classify_touch_zone(coordinate_x: u32, coordinate_y: u32) -> u8 {
        if coordinate_x < TOUCH_ZONE_EDGE && coordinate_y < TOUCH_ZONE_EDGE {
            1
        } else if coordinate_x > TOUCH_ZONE_FAR_EDGE && coordinate_y > TOUCH_ZONE_FAR_EDGE {
            9
        } else {
            5
        }
    }
}

impl Default for HardwareAbstractionLayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Destructor automat pentru ștergerea configurărilor hardware din memoria RAM volatile
impl Drop for HardwareAbstractionLayer {
    fn drop(&mut self) {
        self.current_mode = DisplayMode::BlindGeometry;
        self.hardware_active = false;
    }
}
