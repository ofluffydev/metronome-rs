use metronome_rs::*;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Practical Subdivision Training Examples");
    println!("======================================");

    // Example 1: Learning to play on the beat first
    println!("\n1. Step 1: Get comfortable with quarter notes (60 BPM)");
    play_metronome_for_duration(60.0, Some(4), 6000)?;

    println!("   Now try playing along with eighth notes...");
    thread::sleep(Duration::from_millis(1000));

    // Example 2: Add eighth note subdivisions to feel the space
    println!("\n2. Step 2: Add eighth note subdivisions to feel the beat (60 BPM)");
    println!("   Play on beats 1, 2, 3, 4 - listen to subdivisions on 'e'");
    start_metronome_with_eighth_notes(60.0, Some(4))?;
    thread::sleep(Duration::from_millis(8000)); // 2 full measures
    stop_global_metronome();

    thread::sleep(Duration::from_millis(1000));

    // Example 3: Increase tempo gradually
    println!("\n3. Step 3: Gradually increase tempo with subdivisions");
    let tempos = [70.0, 80.0, 90.0, 100.0];

    for &tempo in &tempos {
        println!("   Practicing at {} BPM with eighth notes", tempo);
        start_metronome_with_eighth_notes(tempo, Some(4))?;
        thread::sleep(Duration::from_millis(4000)); // 1 measure at each tempo
        stop_global_metronome();
        thread::sleep(Duration::from_millis(500));
    }

    // Example 4: Triplet practice
    println!("\n4. Triplet training progression");
    println!("   Learning to feel triplets against quarter notes");

    println!("   a) Quarter notes only (80 BPM)");
    play_metronome_for_duration(80.0, Some(4), 3000)?;

    thread::sleep(Duration::from_millis(500));

    println!("   b) Same tempo with triplet subdivisions");
    println!("      Play quarter notes, listen to triplet subdivisions");
    start_metronome_with_triplets(80.0, Some(4))?;
    thread::sleep(Duration::from_millis(6000));
    stop_global_metronome();

    thread::sleep(Duration::from_millis(1000));

    // Example 5: Advanced rhythm practice
    println!("\n5. Advanced: Sixteenth note precision training");
    println!("   Practice playing on specific sixteenth note subdivisions");

    println!("   Playing at 70 BPM with sixteenth note subdivisions");
    println!("   Try playing on beats 1, the 'e' of 2, the '+' of 3, and the 'a' of 4");
    start_metronome_with_sixteenth_notes(70.0, Some(4))?;
    thread::sleep(Duration::from_millis(8000)); // 2 full measures
    stop_global_metronome();

    thread::sleep(Duration::from_millis(1000));

    // Example 6: Custom practice for specific needs
    println!("\n6. Custom subdivision for polyrhythm practice");
    println!("   5 against 4 polyrhythm training");

    // Create a custom configuration with 5 subdivisions
    println!("   5 subdivisions per beat - practice playing groups of 5");
    start_metronome_with_subdivisions(60.0, Some(4), 5, 0.4)?;
    thread::sleep(Duration::from_millis(6000));
    stop_global_metronome();

    println!("\nTraining complete!");
    println!("\nTips for using subdivisions effectively:");
    println!("- Start slow and gradually increase tempo");
    println!("- Use subdivisions to feel the space between beats");
    println!("- Don't rely on subdivisions forever - gradually remove them");
    println!("- Different subdivision volumes help distinguish main beats");
    println!("- Triplets help with compound time and jazz feel");
    println!("- Sixteenth notes are essential for funk and complex rhythms");

    Ok(())
}
