use anyhow::Ok;
use std::time::Instant;
use std::{thread, time};
use words::spelling_bee::{SpellingBee, SpellingBeeHintedParams, SpellingBeeSimpleParams};

fn main() -> anyhow::Result<()> {
    let game_simple = SpellingBeeSimpleParams::new("abcdefg")?;
    let game_hinted = SpellingBeeHintedParams::new("abcdefg", vec![(b'a', vec![4])], vec![*b"ac"])?;

    let start = Instant::now();
    game_simple.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeSimpleParams dict 1 is: {:?}",
        duration
    );

    let start = Instant::now();
    game_simple.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeSimpleParams dict 2 is: {:?}",
        duration
    );

    let start = Instant::now();
    game_simple.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeSimpleParams dict 3 is: {:?}",
        duration
    );

    let start = Instant::now();
    game_simple.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeSimpleParams dict 4 is: {:?}",
        duration
    );

    let start = Instant::now();
    game_hinted.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeHintedParams dict 1 is: {:?}",
        duration
    );

    let start = Instant::now();
    game_hinted.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeHintedParams dict 2 is: {:?}",
        duration
    );

    let start = Instant::now();
    game_hinted.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeHintedParams dict 3 is: {:?}",
        duration
    );

    let start = Instant::now();
    game_hinted.scan_dict();
    let duration = start.elapsed();

    println!(
        "Time elapsed in SpellingBeeHintedParams dict 4 is: {:?}",
        duration
    );

    let ten_millis = time::Duration::from_secs(60);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
    Ok(())
}
