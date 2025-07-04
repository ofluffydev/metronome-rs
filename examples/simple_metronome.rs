use metronome_rs::Metronome;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Simple Metronome Demo");
    println!("Playing 120 BPM for 5 seconds...");

    let metronome = Metronome::new(120.0, None)?;

    metronome.start()?;
    println!("Metronome started! You should hear steady beats at 120 BPM.");

    // Play for 5 seconds
    thread::sleep(Duration::from_millis(5000));

    metronome.stop();
    println!("Metronome stopped!");

    // Give it a moment to fully stop
    thread::sleep(Duration::from_millis(100));

    Ok(())
}
