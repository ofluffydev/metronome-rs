#[cfg(test)]
use crate::{stop_global_metronome, AccentConfig, Metronome};
#[cfg(test)]
use std::thread;
#[cfg(test)]
use std::time::Duration;

#[test]
fn test_beep() {
    println!("Ensure you hear a beep sound");
    crate::beep().expect("Failed to play beep");
}

#[test]
fn test_metronome_creation() {
    let metronome = Metronome::new(120.0, Some(4)).expect("Failed to create metronome");
    assert_eq!(metronome.bpm(), 120.0);
    assert_eq!(metronome.beats_per_measure(), Some(4));
    assert!(!metronome.is_playing());
}

#[test]
fn test_metronome_simple() {
    println!("Testing simple metronome (120 BPM, no measures) - should hear 3 beats");
    let metronome = Metronome::new(120.0, None).expect("Failed to create metronome");

    metronome.start().expect("Failed to start metronome");
    assert!(metronome.is_playing());

    // Let it play for about 3 beats (1.5 seconds at 120 BPM)
    thread::sleep(Duration::from_millis(1500));

    metronome.stop();
    thread::sleep(Duration::from_millis(100)); // Give it time to stop
    assert!(!metronome.is_playing());
}

#[test]
fn test_metronome_with_measures() {
    println!("Testing metronome with measures (120 BPM, 4 beats per measure) - first beat should be accented");
    let metronome = Metronome::new(120.0, Some(4)).expect("Failed to create metronome");

    metronome.start().expect("Failed to start metronome");

    // Let it play for about 2 measures (4 seconds at 120 BPM)
    thread::sleep(Duration::from_millis(4000));

    metronome.stop();
    thread::sleep(Duration::from_millis(100));
    assert!(!metronome.is_playing());
}

#[test]
fn test_accent_configuration() {
    let metronome = Metronome::new_with_accent(120.0, Some(4), AccentConfig::strong())
        .expect("Failed to create metronome with strong accent");

    // Test that accent config is properly set
    assert_eq!(metronome.accent_config().accent_frequency, 1760.0);
    assert_eq!(metronome.accent_config().regular_frequency, 440.0);
    assert_eq!(metronome.accent_config().accent_duration, 200);
    assert_eq!(metronome.accent_config().regular_duration, 80);
}

#[test]
fn test_accent_config_presets() {
    let default_config = AccentConfig::default();
    assert_eq!(default_config.accent_frequency, 880.0);
    assert_eq!(default_config.regular_frequency, 440.0);

    let subtle_config = AccentConfig::subtle();
    assert_eq!(subtle_config.accent_frequency, 660.0);

    let strong_config = AccentConfig::strong();
    assert_eq!(strong_config.accent_frequency, 1760.0);
    assert_eq!(strong_config.accent_duration, 200);
}

#[test]
fn test_global_metronome_singleton() {
    println!("Testing global metronome singleton behavior");

    // Make sure we start clean
    stop_global_metronome();
    thread::sleep(Duration::from_millis(500)); // Increased wait time

    let metronome1 = Metronome::new(120.0, None).expect("Failed to create metronome 1");
    let metronome2 = Metronome::new(140.0, Some(3)).expect("Failed to create metronome 2");

    // Start first metronome
    metronome1.start().expect("Failed to start metronome 1");
    assert!(metronome1.is_playing());

    thread::sleep(Duration::from_millis(500)); // Increased wait time

    // Start second metronome - should stop the first one
    metronome2.start().expect("Failed to start metronome 2");
    thread::sleep(Duration::from_millis(300)); // Increased wait time for the stop to propagate

    // Check states
    assert!(!metronome1.is_playing());
    assert!(metronome2.is_playing());

    thread::sleep(Duration::from_millis(500));

    // Stop all metronomes
    stop_global_metronome();
    thread::sleep(Duration::from_millis(300)); // Increased wait time
    assert!(!metronome2.is_playing());

    // Clean up for next test
    thread::sleep(Duration::from_millis(200));
}

#[test]
fn test_metronome_start_stop_simple() {
    println!("Testing simple start/stop");

    // Make sure we start clean
    stop_global_metronome();
    thread::sleep(Duration::from_millis(100));

    let metronome = Metronome::new(120.0, None).expect("Failed to create metronome");

    // Start metronome
    metronome.start().expect("Failed to start metronome");
    assert!(metronome.is_playing());

    // Let it play briefly
    thread::sleep(Duration::from_millis(600)); // Just over one beat at 120 BPM

    // Stop metronome
    metronome.stop();
    thread::sleep(Duration::from_millis(100));
    assert!(!metronome.is_playing());

    println!("Simple start/stop test completed");
}

#[test]
fn test_wave_type_configurations() {
    use crate::{AccentConfig, WaveType};

    // Test default wave type
    let default_config = AccentConfig::default();
    assert_eq!(default_config.accent_wave_type, WaveType::Sine);
    assert_eq!(default_config.regular_wave_type, WaveType::Sine);

    // Test single wave type configuration
    let square_config = AccentConfig::with_wave_type(WaveType::Square);
    assert_eq!(square_config.accent_wave_type, WaveType::Square);
    assert_eq!(square_config.regular_wave_type, WaveType::Square);

    // Test mixed wave type configuration
    let mixed_config = AccentConfig::with_wave_types(WaveType::Triangle, WaveType::Sawtooth);
    assert_eq!(mixed_config.accent_wave_type, WaveType::Triangle);
    assert_eq!(mixed_config.regular_wave_type, WaveType::Sawtooth);

    // Test preset configurations
    let strong_square_config = AccentConfig::strong_square();
    assert_eq!(strong_square_config.accent_wave_type, WaveType::Square);
    assert_eq!(strong_square_config.regular_wave_type, WaveType::Sine);

    let subtle_triangle_config = AccentConfig::subtle_triangle();
    assert_eq!(subtle_triangle_config.accent_wave_type, WaveType::Triangle);
    assert_eq!(subtle_triangle_config.regular_wave_type, WaveType::Triangle);
}

#[test]
fn test_metronome_with_wave_types() {
    use crate::{AccentConfig, WaveType};

    println!("Testing metronome with square wave type - should hear harsh digital sounds");
    let square_config = AccentConfig::with_wave_type(WaveType::Square);
    let metronome = Metronome::new_with_accent(120.0, Some(4), square_config)
        .expect("Failed to create metronome with square wave");

    metronome.start().expect("Failed to start metronome");
    thread::sleep(Duration::from_millis(1500)); // About 3 beats
    metronome.stop();
    thread::sleep(Duration::from_millis(100));
    assert!(!metronome.is_playing());
}

#[test]
fn test_high_level_helper_functions() {
    use crate::{
        start_metronome_with_time_signature, start_performance_metronome, start_practice_metronome,
        start_simple_metronome, stop_global_metronome,
    };

    println!("Testing high-level helper functions");

    // Ensure clean state
    stop_global_metronome();
    thread::sleep(Duration::from_millis(300));

    // Test simple metronome helper
    start_simple_metronome(120.0).expect("Failed to start simple metronome");
    thread::sleep(Duration::from_millis(500));
    stop_global_metronome();
    thread::sleep(Duration::from_millis(200));

    // Test time signature helper
    start_metronome_with_time_signature(100.0, 4)
        .expect("Failed to start time signature metronome");
    thread::sleep(Duration::from_millis(500));
    stop_global_metronome();
    thread::sleep(Duration::from_millis(200));

    // Test practice metronome helper
    start_practice_metronome(80.0, 3).expect("Failed to start practice metronome");
    thread::sleep(Duration::from_millis(500));
    stop_global_metronome();
    thread::sleep(Duration::from_millis(200));

    // Test performance metronome helper
    start_performance_metronome(90.0, 4).expect("Failed to start performance metronome");
    thread::sleep(Duration::from_millis(500));
    stop_global_metronome();
    thread::sleep(Duration::from_millis(200));

    println!("High-level helper functions test completed");
}

#[test]
fn test_timed_metronome_functions() {
    use crate::{play_custom_metronome_for_duration, play_metronome_for_duration, AccentConfig};

    println!("Testing timed metronome functions");

    // Test basic timed metronome
    play_metronome_for_duration(120.0, Some(4), 800).expect("Failed to play timed metronome");

    // Test custom timed metronome
    let config = AccentConfig::subtle();
    play_custom_metronome_for_duration(100.0, Some(3), config, 600)
        .expect("Failed to play custom timed metronome");

    println!("Timed metronome functions test completed");
}
