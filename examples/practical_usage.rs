use metronome_rs::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Practical Metronome Usage Examples");
    println!("==================================");

    // Example 1: Quick practice session
    println!("\n1. Quick 30-second practice session at 80 BPM in 4/4 time");
    play_metronome_for_duration(80.0, Some(4), 30000)?;
    println!("   Practice session completed!");

    thread::sleep(Duration::from_millis(1000));

    // Example 2: Background metronome while doing other work
    println!("\n2. Starting background metronome for warm-up exercises");
    start_practice_metronome(100.0, 4)?;

    // Simulate doing other work
    for i in 1..=5 {
        println!(
            "   Warm-up exercise {} (metronome playing in background)",
            i
        );
        thread::sleep(Duration::from_millis(1200)); // Slightly longer than one beat
    }

    stop_global_metronome();
    println!("   Warm-up completed, metronome stopped");

    thread::sleep(Duration::from_millis(1000));

    // Example 3: Changing tempo during practice
    println!("\n3. Progressive tempo practice");

    let tempos = [60.0, 80.0, 100.0, 120.0];
    for &tempo in &tempos {
        println!("   Playing at {} BPM for 3 seconds", tempo);
        play_metronome_for_duration(tempo, Some(4), 3000)?;
        thread::sleep(Duration::from_millis(500)); // Brief pause between tempos
    }

    println!("   Progressive tempo practice completed!");

    thread::sleep(Duration::from_millis(1000));

    // Example 4: Different time signatures
    println!("\n4. Practicing different time signatures");

    let time_signatures = [
        (4, "4/4 (common time)"),
        (3, "3/4 (waltz time)"),
        (2, "2/4 (march time)"),
        (6, "6/8 (compound time)"),
    ];

    for &(beats, name) in &time_signatures {
        println!("   {} - playing for 4 seconds", name);
        start_performance_metronome(120.0, beats)?;
        thread::sleep(Duration::from_millis(4000));
        stop_global_metronome();
        thread::sleep(Duration::from_millis(500));
    }

    // Example 5: Custom sound for specific practice
    println!("\n5. Custom metronome for electronic music practice");
    let electronic_config = AccentConfig::new(
        800.0,              // Sharp high accent
        400.0,              // Lower regular beats
        100,                // Short accent duration
        80,                 // Even shorter regular duration
        WaveType::Square,   // Electronic square wave accent
        WaveType::Triangle, // Softer triangle regular beats
    );

    println!("   Playing electronic-style metronome for 5 seconds");
    play_custom_metronome_for_duration(128.0, Some(4), electronic_config, 5000)?;

    println!("\nAll examples completed!");
    println!("\nKey takeaways:");
    println!("- Use play_*_for_duration() for timed practice sessions");
    println!("- Use start_*() + stop_global_metronome() for background operation");
    println!("- Use preset functions (practice/performance) for common scenarios");
    println!("- Use custom configurations for specialized sound requirements");

    Ok(())
}
