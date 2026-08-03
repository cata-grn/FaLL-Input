// tests/memory_leak_test.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Transient memory leak and residue verification test suite.
//! Core Architect & Inventor: R.C.F. (2026)

mod common;

use fall_input::core_engine::MultiTapEngine;
use fall_input::memory_manager::VolatileMemoryScrubber;
use common::MockHardwareFixture;

#[test]
fn test_immediate_memory_zeroization_on_drop() {
    // 1. Inițializarea structurilor în mediu Sandbox controlat
    let fixture = MockHardwareFixture::setup_sandbox();
    let mut scrubber = VolatileMemoryScrubber::new();
    
    // Pointer către o zonă care va fi curățată automat
    let mut active_pointer: Option<MultiTapEngine> = Some(MultiTapEngine::new());

    // 2. Blocarea memoriei și simularea scrierii de date critice
    scrubber.lock_memory_zone();
    if let Some(ref mut engine) = active_pointer {
        let touch = fixture.create_mock_touch(500, 500, 450);
        let digit = fixture.hal.process_raw_hardware_input(&touch).unwrap();
        engine.register_hardware_pulse(digit, fixture.simulated_time_ms).unwrap();
    }

    // Confirmă că în timpul rulării, memoria nu este raportată ca fiind curată
    assert!(!scrubber.verify_cleanliness_status());

    // 3. Executarea operațiunii DROP (Variabila părăsește contextul logic/Este distrusă)
    let _ = active_pointer.take();
    scrubber.force_immediate_scrub();

    // 4. Validarea Matematică a Curățeniei RAM: Verificăm ștergerea totală
    // Statusul trebuie să devină TRUE (Imunitate la atacuri tip Memory Dumping)
    assert!(scrubber.verify_cleanliness_status(), "FaLL-Security Failure: Residual data traces detected in volatile memory layouts.");
}
impl Default for StructName {
    fn default() -> Self {
        Self::new()
    }
}
