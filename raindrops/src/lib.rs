pub fn raindrops(n: i32) -> String {
    let factors_sounds_map = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let sound: String = factors_sounds_map
        .iter()
        .filter(|(factor, _sound)| n % factor == 0)
        .map(|(_factor, sound)| *sound)
        .collect();

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}
