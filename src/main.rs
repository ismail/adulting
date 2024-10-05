use rand::thread_rng;
use rand::Rng;

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

    let mut rng = thread_rng();
    let rule_index = rng.gen_range(0..rules.len());

    println!("{}. {}", rule_index + 1, rules[rule_index]);
}
