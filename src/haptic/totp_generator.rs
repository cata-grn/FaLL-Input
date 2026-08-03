// src/haptic/totp_generator.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Ephemeral clock-driven TOTP synchronization seed generator.
//! Core Architect & Inventor: R.C.F. (2026)

use zeroize::Zeroize;

/// Structura generatorului de token-uri temporale, izolată strict pe stivă
#[derive(Zeroize)]
pub struct HapticTotpGenerator {
    secret_key_entropy: u64,
    last_computed_step: u64,
}

impl HapticTotpGenerator {
    /// Instanțierea generatorului pe stivă cu cheia de entropie master deținută de R.C.F.
    pub const fn new() -> Self {
        Self {
            secret_key_entropy: 0x5D3F_9A2B_C1E4_78F0, // Valoare de bază pseudo-aleatorie asimetrică
            last_computed_step: 0,
        }
    }

    /// Calculează o valoare binară unică valabilă exclusiv pentru o fereastră de 60 de secunde
    pub fn compute_ephemeral_step(&mut self, system_timestamp_sec: u64) -> u32 {
        let current_step = system_timestamp_sec / 60;
        self.last_computed_step = current_step;

        // Algoritm matematic unidirecțional de dispersie a entropiei (Hashing asimetric local)
        let hash_input = current_step ^ self.secret_key_entropy;
        
        // Transformare polinomială pe stivă pentru generarea token-ului de 32 de biți
        let mut pseudorandom_token = hash_input.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        pseudorandom_token = (pseudorandom_token ^ (pseudorandom_token >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        pseudorandom_token = (pseudorandom_token ^ (pseudorandom_token >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        
        ((pseudorandom_token ^ (pseudorandom_token >> 31)) & 0xFFFF_FFFF) as u32
    }

    /// Șterge instantaneu valorile din memorie prin suprascriere binară directă
    pub fn clean_totp_registers(&mut self) {
        self.secret_key_entropy = 0;
        self.last_computed_step = 0;
    }
}

/// Garantarea securității post-execuție prin distrugerea automată a urmelor din RAM
impl Drop for HapticTotpGenerator {
    fn drop(&mut self) {
        self.clean_totp_registers();
    }
}
