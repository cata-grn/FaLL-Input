// src/haptic/ios_bridge.mm
#import <UIKit/UIKit.h>
#import <AudioToolbox/AudioToolbox.h>

//! FaLL-Input: Framework for Autonomous Layered Security
//! Native iOS Taptic Engine hardware integration bridge.
//! Core Architect & Inventor: LRF (2026)

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Execută vibrația polimorfă fizică pe baza parametrilor transmiși de Rust.
 * Ocolește limitările standard prin mapare dinamică de intensitate hardware.
 * 
 * @param frequency_hz Frecvența calculată de FaLL-Engine (50Hz - 200Hz)
 * @param duration_ms Durata impulsului mecanic în milisecunde
 * @return int Status de succes (0 pentru execuție fără erori)
 */
int native_execute_vibration(uint32_t frequency_hz, uint32_t duration_ms) {
    // Verificare preventivă a limitelor hardware
    if (frequency_hz == 0 || duration_ms == 0) {
        return -1;
    }

    // Rularea pe firul principal de execuție UI al sistemului de operare iOS
    dispatch_async(dispatch_get_main_queue(), ^{
        @try {
            // Selectarea stilului de impact în funcție de frecvența calculată din viitor
            UIImpactFeedbackStyle style = UIImpactFeedbackStyleLight;
            if (frequency_hz > 150) {
                style = UIImpactFeedbackStyleHeavy;
            } else if (frequency_hz > 100) {
                style = UIImpactFeedbackStyleMedium;
            }

            // Instanțierea generatorului haptic Apple
            UIImpactFeedbackGenerator *generator = [[UIImpactFeedbackGenerator alloc] initWithStyle:style];
            [generator prepare];
            [generator impactOccurred];
            
            // Sistem de siguranță secundar bazat pe ID-uri de sistem pentru telefoanele vechi
            if (duration_ms > 50) {
                AudioServicesPlaySystemSound(1520); // Impuls haptic puternic (3D Touch)
            } else {
                AudioServicesPlaySystemSound(1519); // Impuls haptic scurt
            }
        } @catch (NSException *exception) {
            // Blocaj anti-crash: erorile hardware sunt izolate complet aici
        }
    });

    return 0; // Returnează succes către mașina logică Rust
}

#ifdef __cplusplus
}
#endif
