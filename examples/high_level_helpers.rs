use metronome_rs::{
    AccentConfig, WaveType, play_custom_metronome_for_duration, play_metronome_for_duration,
    start_custom_metronome, start_metronome_with_time_signature, start_performance_metronome,
    start_practice_metronome, start_simple_metronome, stop_global_metronome,
};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("High-Level Metronome Helpers Demo");
    println!("=================================");

    // Demo 1: Simple metronome - plays indefinitely until stopped
    println!("\n1. Simple metronome (120 BPM) - background operation");
    println!("   Starting for 3 seconds...");
    start_simple_metronome(120.0)?;
    thread::sleep(Duration::from_millis(3000));
    stop_global_metronome();

    println!("   Stopped. Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 2: Metronome with time signature - plays indefinitely until stopped
    println!("\n2. 4/4 time metronome (100 BPM) - background operation");
    println!("   Starting for 4 seconds...");
    start_metronome_with_time_signature(100.0, 4)?;
    thread::sleep(Duration::from_millis(4000));
    stop_global_metronome();

    println!("   Stopped. Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 3: Timed metronome - automatically stops after duration
    println!("\n3. Timed metronome (140 BPM, 3/4 time) - auto-stop after 3 seconds");
    println!("   This will block and automatically stop...");
    play_metronome_for_duration(140.0, Some(3), 3000)?;

    println!("   Finished. Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 4: Custom metronome with square waves - plays indefinitely
    println!("\n4. Custom metronome with square waves (110 BPM, 4/4 time)");
    println!("   Starting for 3 seconds...");
    let square_config = AccentConfig::with_wave_type(WaveType::Square);
    start_custom_metronome(110.0, Some(4), square_config)?;
    thread::sleep(Duration::from_millis(3000));
    stop_global_metronome();

    println!("   Stopped. Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 5: Custom timed metronome with triangle waves
    println!("\n5. Custom timed metronome with triangle waves (90 BPM, 2/4 time)");
    println!("   Playing for 3 seconds with auto-stop...");
    let triangle_config = AccentConfig::subtle_triangle();
    play_custom_metronome_for_duration(90.0, Some(2), triangle_config, 3000)?;

    println!("   Finished. Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 6: Practice metronome preset - subtle accents
    println!("\n6. Practice metronome preset (80 BPM, 4/4 time) - subtle accents");
    println!("   Starting for 4 seconds...");
    start_practice_metronome(80.0, 4)?;
    thread::sleep(Duration::from_millis(4000));
    stop_global_metronome();

    println!("   Stopped. Waiting 1 second...");
    thread::sleep(Duration::from_millis(1000));

    // Demo 7: Performance metronome preset - strong accents
    println!("\n7. Performance metronome preset (130 BPM, 3/4 time) - strong accents");
    println!("   Starting for 4 seconds...");
    start_performance_metronome(130.0, 3)?;
    thread::sleep(Duration::from_millis(4000));
    stop_global_metronome();

    println!("\nDemo complete!");
    println!("\nUsage patterns demonstrated:");
    println!("- Background metronomes: start_*() functions + stop_global_metronome()");
    println!("- Timed metronomes: play_*_for_duration() functions (blocking)");
    println!("- Presets: practice_metronome() and performance_metronome()");
    println!("- Custom configurations: Use AccentConfig for full control");

    Ok(())
}
