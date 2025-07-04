use metronome_rs::Metronome;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Metronome with Measures Demo");
    println!("Playing 120 BPM with 4 beats per measure for 8 seconds...");
    println!("The first beat of each measure will be accented (higher pitch).");

    let metronome = Metronome::new(120.0, Some(4))?;

    metronome.start()?;
    println!("Metronome started! Listen for the accented first beat of each measure.");

    // Play for 8 seconds (should hear 2 complete measures at 120 BPM)
    thread::sleep(Duration::from_millis(8000));

    metronome.stop();
    println!("Metronome stopped!");

    // Give it a moment to fully stop
    thread::sleep(Duration::from_millis(100));

    Ok(())
}
