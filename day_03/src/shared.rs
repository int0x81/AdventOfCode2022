pub fn get_priority(character: &char) -> usize {
    let unicode_value: u32 = *character as u32;

    if unicode_value >= 0x61 && unicode_value <= 0x7a {
        return (unicode_value - 0x60).try_into().unwrap();
    }

    if unicode_value >= 0x41 && unicode_value <= 0x5a {
        return (unicode_value - 0x40 + 26).try_into().unwrap();
    }

    panic!("Unable to get priority from character");
}