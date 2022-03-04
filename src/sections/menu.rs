use super::*;

pub fn menu() {
    print_title!("Menu");
    printlnc!("What you want to learn ?\n", YELLOW);
    match choose(&[
        "Kanji",
        "Hiragana (Coming soon)",
        "Katakana (Coming soon)",
        "Exit"
    ]){
        Some(1) => kanji(),
        Some(2) => todo!(),
        Some(3) => todo!(),
        Some(4) => {},
        _ => menu()
    }
}