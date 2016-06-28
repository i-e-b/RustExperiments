
//! A simple multply-with-carry PRNG from
//! (http://cas.ee.ic.ac.uk/people/dt10/research/rngs-gpu-mwc64x.html)
//!
//! Rust has it's own random number generator in the standard library,
//! use that rather than this.
//! This is for learning about number size casting and time

use std::time;

fn mwc64x(state: &mut u64) -> u32 {
    const MUL_X: u64 = 4294883355;

    let c = (*state) >> 32;
    let x = (*state) & 0xFFFFFFFF;
    *state = (x * MUL_X) + c;
    return (x^c) as u32;
}

/// use the system clock to get a seed value
fn now64() -> u64 {
    let now = time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Negative time");
    let mut ticks = now.as_secs();
    let spins = 5 + (ticks & 0xF);
    for _ in 1..spins { // must use `_` to avoid 'unused variable' warning
        mwc64x(&mut ticks); // amplify small time differences
    }
    return ticks;
}

fn main() {
    let mut state : u64 = now64();
    println!("Start state: {}", state);

    println!("{}", mwc64x(&mut state));
    println!("{}", mwc64x(&mut state));
    println!("{}", mwc64x(&mut state));
    println!("{}", mwc64x(&mut state));
    println!("{}", mwc64x(&mut state));
    println!("{}", mwc64x(&mut state));
    println!("{}", mwc64x(&mut state));
    println!("{}", mwc64x(&mut state));
    println!("End state: {}", state);
}
