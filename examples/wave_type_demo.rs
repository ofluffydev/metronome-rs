use metronome_rs::{AccentConfig, Metronome, WaveType};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Wave Type Demo - Testing different wave types for metronome sounds");
    println!("Each test plays for about 2 seconds (4 beats at 120 BPM)");

    // Test 1: Sine wave (default)
    println!("\n1. Testing Sine wave (smooth, pure tone)");
    let sine_config = AccentConfig::with_wave_type(WaveType::Sine);
    let metronome1 = Metronome::new_with_accent(120.0, Some(4), sine_config)?;
    metronome1.start()?;
    thread::sleep(Duration::from_millis(2000));
    metronome1.stop();
    thread::sleep(Duration::from_millis(500));

    // Test 2: Square wave
    println!("2. Testing Square wave (harsh, digital sound)");
    let square_config = AccentConfig::with_wave_type(WaveType::Square);
    let metronome2 = Metronome::new_with_accent(120.0, Some(4), square_config)?;
    metronome2.start()?;
    thread::sleep(Duration::from_millis(2000));
    metronome2.stop();
    thread::sleep(Duration::from_millis(500));

    // Test 3: Sawtooth wave
    println!("3. Testing Sawtooth wave (bright, buzzy tone)");
    let sawtooth_config = AccentConfig::with_wave_type(WaveType::Sawtooth);
    let metronome3 = Metronome::new_with_accent(120.0, Some(4), sawtooth_config)?;
    metronome3.start()?;
    thread::sleep(Duration::from_millis(2000));
    metronome3.stop();
    thread::sleep(Duration::from_millis(500));

    // Test 4: Triangle wave
    println!("4. Testing Triangle wave (softer than square, warmer than sine)");
    let triangle_config = AccentConfig::with_wave_type(WaveType::Triangle);
    let metronome4 = Metronome::new_with_accent(120.0, Some(4), triangle_config)?;
    metronome4.start()?;
    thread::sleep(Duration::from_millis(2000));
    metronome4.stop();
    thread::sleep(Duration::from_millis(500));

    // Test 5: Mixed wave types (square accent, sine regular)
    println!("5. Testing Mixed wave types (Square accents, Sine regular beats)");
    let mixed_config = AccentConfig::strong_square();
    let metronome5 = Metronome::new_with_accent(120.0, Some(4), mixed_config)?;
    metronome5.start()?;
    thread::sleep(Duration::from_millis(2000));
    metronome5.stop();
    thread::sleep(Duration::from_millis(500));

    println!("\nWave type demo completed!");
    Ok(())
}
