use metronome_rs::{stop_global_metronome, Metronome};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Multiple Metronomes Demo");
    println!("This demonstrates that only one metronome can play at a time (global singleton).");

    println!("\n1. Creating first metronome at 100 BPM...");
    let metronome1 = Metronome::new(100.0, None)?;
    metronome1.start()?;
    println!("First metronome started (100 BPM). Listen to the slow beats...");

    // Play for 3 seconds
    thread::sleep(Duration::from_millis(3000));

    println!("\n2. Creating second metronome at 160 BPM...");
    let metronome2 = Metronome::new(160.0, Some(3))?;
    metronome2.start()?;
    println!("Second metronome started (160 BPM with measures). The first metronome should have stopped automatically.");
    println!("You should now hear faster beats with accents every 3 beats.");

    // Play for 4 seconds
    thread::sleep(Duration::from_millis(4000));

    println!("\n3. Stopping all metronomes...");
    stop_global_metronome();

    // Give it a moment to fully stop
    thread::sleep(Duration::from_millis(200));

    println!("All metronomes stopped!");
    println!("First metronome playing: {}", metronome1.is_playing());
    println!("Second metronome playing: {}", metronome2.is_playing());

    Ok(())
}
