use metronome_rs::{AccentConfig, Metronome};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Metronome Subdivision Demo");
    println!("=========================");
    println!("Demonstrates audible and musical subdivisions with different patterns");

    // Test 1: Eighth notes (should be clearly audible)
    println!("\n1. Eighth notes (2 subdivisions per beat) at 100 BPM");
    println!("   Listen for: LOUD-soft-LOUD-soft pattern");
    let eighth_config = AccentConfig::with_eighth_notes();
    let metronome1 = Metronome::new_with_accent(100.0, Some(4), eighth_config)?;
    metronome1.start()?;
    thread::sleep(Duration::from_millis(4000)); // About 2 measures
    metronome1.stop();

    println!("\n   Waiting 2 seconds...");
    thread::sleep(Duration::from_millis(2000));

    // Test 2: Sixteenth notes (should be audible with square wave)
    println!("\n2. Sixteenth notes (4 subdivisions per beat) at 80 BPM");
    println!("   Listen for: LOUD-soft-soft-soft pattern with bright subdivision clicks");
    let sixteenth_config = AccentConfig::with_sixteenth_notes();
    let metronome2 = Metronome::new_with_accent(80.0, Some(4), sixteenth_config)?;
    metronome2.start()?;
    thread::sleep(Duration::from_millis(6000)); // About 2 measures at slower tempo
    metronome2.stop();

    println!("\n   Waiting 2 seconds...");
    thread::sleep(Duration::from_millis(2000));

    // Test 3: Triplets (should sound musical)
    println!("\n3. Triplets (3 subdivisions per beat) at 90 BPM");
    println!("   Listen for: LOUD-soft-soft pattern with triangle wave subdivisions");
    let triplet_config = AccentConfig::with_triplets();
    let metronome3 = Metronome::new_with_accent(90.0, Some(4), triplet_config)?;
    metronome3.start()?;
    thread::sleep(Duration::from_millis(5000)); // About 2 measures
    metronome3.stop();

    println!("\n   Waiting 2 seconds...");
    thread::sleep(Duration::from_millis(2000));

    // Test 4: Custom high-volume subdivisions
    println!("\n4. Custom high-volume subdivisions (6 per beat) at 60 BPM");
    println!("   Listen for: Very frequent subdivision clicks at high volume");
    let custom_config = AccentConfig::with_custom_subdivisions(6, 698.46, 0.8); // F5 note at 80% volume
    let metronome4 = Metronome::new_with_accent(60.0, Some(4), custom_config)?;
    metronome4.start()?;
    thread::sleep(Duration::from_millis(4000)); // 1 measure at slow tempo
    metronome4.stop();

    println!("\nDemo complete!");
    println!("\nImprovements made:");
    println!("- Higher frequencies for subdivisions (C5 instead of E4)");
    println!("- Increased volumes (0.55-0.7 instead of 0.3-0.5)");
    println!("- Longer durations for better audibility");
    println!("- Square waves for sixteenth notes (more cutting)");
    println!("- Triangle waves for triplets (more musical)");

    Ok(())
}
