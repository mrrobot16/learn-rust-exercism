const VERSES: [&'static str; 3] = [
"No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
"1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
"2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n",
];

pub fn verse(n: u32) -> String {
    match n {
        0 => VERSES[n as usize].to_string(),
        1 => VERSES[n as usize].to_string(),
        2 => VERSES[n as usize].to_string(),
        _ => {
            build_verse(n)
        }
    }
}

fn build_verse(n: u32) -> String {
    let base_verse = "{0} bottles of beer on the wall, {0} bottles of beer.\n\
                   Take one down and pass it around, {1} bottles of beer \
                   on the wall.\n";
    let verse = format!(
        "{0} bottles of beer on the wall, {0} bottles of beer.\n\
        Take one down and pass it around, {1} bottles of beer \
        on the wall.\n",
        n,
        n-1
    );
    verse.to_string()
}

pub fn sing(start: u32, end: u32) -> String {
    let range = (end..start + 1).rev();
    let closure = |x| verse(x);
    let song = range.map(closure).collect::<Vec<_>>().join("\n");
    song
}
// Cleaner community solutions

pub fn verse(n: u32) -> String {
    let parts = |n| match n {
        1 => ("1 bottle of beer".to_string(), "Take it down and pass it around"),
        0 => ("no more bottles of beer".to_string(), "Go to the store and buy some more"),
        _ => (format!("{} bottles of beer", n), "Take one down and pass it around"),
    };
    let cap = |mut s: String| { s.split_at_mut(1).0.make_ascii_uppercase(); s };
    let wall = "on the wall";
    let (bottles, pass) = parts(n);
    let b_wall = format!("{} {}", parts(if n > 0 { n-1 } else { 99 }).0, wall);
    cap(format!("{} {}, {}.\n{}, {}.\n", bottles, wall, bottles, pass, b_wall))
}
pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(|n| verse(n)).collect::<Vec<_>>().join("\n")
}
