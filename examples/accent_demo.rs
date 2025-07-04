use metronome_rs::{AccentConfig, Metronome};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Metronome Accent Demo");
    println!("====================");

    // Demo 1: Standard metronome without accents
    println!("\n1. Standard metronome (120 BPM) - no accents");
    println!("   Playing for 3 seconds...");
    let metronome1 = Metronome::new(120.0, None)?;
    metronome1.start()?;
    thread::sleep(Duration::from_millis(3000));
    metronome1.stop();

    println!("\n   Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 2: 4/4 time with default accent
    println!("\n2. 4/4 time metronome (100 BPM) with default accent");
    println!("   First beat of each measure is accented (higher pitch, longer duration)");
    println!("   Playing for 6 seconds (should hear 1.5 measures)...");
    let metronome2 = Metronome::new(100.0, Some(4))?;
    metronome2.start()?;
    thread::sleep(Duration::from_millis(6000));
    metronome2.stop();

    println!("\n   Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 3: 3/4 time (waltz) with subtle accent
    println!("\n3. 3/4 time metronome (120 BPM) with subtle accent");
    println!("   Using subtle accent configuration");
    println!("   Playing for 6 seconds (should hear 2.4 measures)...");
    let metronome3 = Metronome::new_with_accent(120.0, Some(3), AccentConfig::subtle())?;
    metronome3.start()?;
    thread::sleep(Duration::from_millis(6000));
    metronome3.stop();

    println!("\n   Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 4: Strong accent example
    println!("\n4. 4/4 time metronome (80 BPM) with strong accent");
    println!("   Very pronounced accent on first beat");
    println!("   Playing for 6 seconds (should hear 1.2 measures)...");
    let metronome4 = Metronome::new_with_accent(80.0, Some(4), AccentConfig::strong())?;
    metronome4.start()?;
    thread::sleep(Duration::from_millis(6000));
    metronome4.stop();

    println!("\n   Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 5: Custom accent configuration
    println!("\n5. Custom accent configuration");
    println!("   2/4 time with custom frequencies and durations");
    println!("   Playing for 4 seconds (should hear 2 measures)...");
    let custom_accent = AccentConfig::new(
        523.25,                       // C5 note for accent
        329.63,                       // E4 note for regular beats
        180,                          // Longer accent duration
        90,                           // Shorter regular duration
        metronome_rs::WaveType::Sine, // Sine wave for accent
        metronome_rs::WaveType::Sine, // Sine wave for regular beats
    );
    let metronome5 = Metronome::new_with_accent(120.0, Some(2), custom_accent)?;
    metronome5.start()?;
    thread::sleep(Duration::from_millis(4000));
    metronome5.stop();

    println!("\nDemo complete!");
    println!("\nAccent patterns explained:");
    println!("- No accents: All beats sound the same");
    println!("- 4/4 time: Strong beat on 1, weak beats on 2, 3, 4");
    println!("- 3/4 time: Strong beat on 1, weak beats on 2, 3 (waltz time)");
    println!("- 2/4 time: Strong beat on 1, weak beat on 2");

    Ok(())
}
