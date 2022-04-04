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
