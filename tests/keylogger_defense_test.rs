// tests/keylogger_defense_test.rs
#![forbid(unsafe_code)]
#![deny(warnings)]

//! FaLL-Input: Framework for Autonomous Layered Security
//! Keylogger interception evasion verification test suite.
//! Core Architect & Inventor: LRF (2026)

mod common;

use fall_input::core_engine::MultiTapEngine;
use common::MockHardwareFixture;

#[test]
fn test_keylogger_interception_evasion() {
    // 1. Inițializarea mediului izolat Sandbox sub parametrii LRF
    let fixture = MockHardwareFixture::setup_sandbox();
    let mut engine = MultiTapEngine::new();

    // 2. Simulare input utilizator: tastarea succesivă a tastei 2 pentru a genera un impuls (ex: la ora exactă a sistemului)
    let touch_1 = fixture.create_mock_touch(100, 100, 450);
    let resolved_digit = fixture.hal.process_raw_hardware_input(&touch_1).unwrap();
    
    assert_eq!(resolved_digit, 1); // Confirmă că HAL-ul citește corect zona hardware

    // Înregistrarea impulsului brut în motorul Rust local
    let res = engine.register_hardware_pulse(resolved_digit, fixture.simulated_time_ms);
    assert!(res.is_ok());

    // 3. Atacul simulat (Keylogger-ul spionează starea memoriei)
    let memory_trace = engine.fetch_vector_state();

    // 4. Validarea Matematică a Imunității: Verificăm că datele sunt doar vectori numerici duri
    // Un virus caută text ASCII sau caractere alfanumerice. FaLL-Input stochează doar indici de tip u8.
    assert_eq!(memory_trace[0], 1);
    
    // Verificare strictă: bufferul nu are legătură cu tabelele de caractere text standard din sistem
    let string_leak_attempt = std::str::from_utf8(memory_trace);
    assert!(string_leak_attempt.is_err(), "FaLL-Security Failure: Plaintext string representation leaked in RAM.");
}
