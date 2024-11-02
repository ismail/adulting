use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LCG {
    multiplier: u32,
    increment: u32,
    modulus: u32,
    current_state: u32,
}

impl LCG {
    /// Creates a new LCG with default parameters and a time-based seed
    pub fn new() -> Self {
        let seed = Self::generate_seed();
        Self {
            // Using better parameters from "Numerical Recipes"
            multiplier: 1664525,
            increment: 1013904223,
            modulus: u32::MAX,
            current_state: seed,
        }
    }

    /// Generates a seed based on system time
    fn generate_seed() -> u32 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                // Combine seconds and nanos for better entropy
                let seconds = duration.as_secs() as u32;
                let nanos = duration.subsec_nanos();
                seconds.wrapping_mul(1_000_000_000).wrapping_add(nanos)
            }
            Err(_) => 42, // Fallback seed
        }
    }

    /// Generates the next random number in the sequence
    fn next_state(&mut self) -> u32 {
        self.current_state = self
            .multiplier
            .wrapping_mul(self.current_state)
            .wrapping_add(self.increment);
        self.current_state
    }

    /// Generates a random number in range [0, max)
    pub fn next(&mut self, max: u32) -> u32 {
        if max == 0 {
            panic!("max must be greater than 0");
        }

        // Use rejection sampling to avoid modulo bias
        let threshold = (self.modulus - self.modulus % max) - 1;
        loop {
            let value = self.next_state();
            if value <= threshold {
                return value % max;
            }
        }
    }
}

fn main() {
    // From https://www.openculture.com/2018/02/the-25-principles-for-adult-behavior.html
    let rules = [
        "Be patient. No matter what.",
        "Don’t badmouth: Assign responsibility, not blame. Say nothing of another you wouldn’t say to him.",
        "Never assume the motives of others are, to them, less noble than yours are to you.",
        "Expand your sense of the possible.",
        "Don’t trouble yourself with matters you truly cannot change.",
        "Expect no more of anyone than you can deliver yourself.",
        "Tolerate ambiguity.",
        "Laugh at yourself frequently.",
        "Concern yourself with what is right rather than who is right.",
        "Never forget that, no matter how certain, you might be wrong.",
        "Give up blood sports.",
        "Remember that your life belongs to others as well. Don’t risk it frivolously.",
        "Never lie to anyone for any reason. (Lies of omission are sometimes exempt.)",
        "Learn the needs of those around you and respect them.",
        "Avoid the pursuit of happiness. Seek to define your mission and pursue that.",
        "Reduce your use of the first personal pronoun.",
        "Praise at least as often as you disparage.",
        "Admit your errors freely and soon.",
        "Become less suspicious of joy.",
        "Understand humility.",
        "Remember that love forgives everything.",
        "Foster dignity.",
        "Live memorably.",
        "Love yourself.",
        "Endure.",
    ];

    let mut rng = LCG::new();
    let rule_index = rng.next(rules.len() as u32) as usize;

    let _ = writeln!(
        std::io::stdout(),
        "{}. {}",
        rule_index + 1,
        rules[rule_index]
    );
}
