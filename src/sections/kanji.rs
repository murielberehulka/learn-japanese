use super::*;

pub fn kanji() {
    print_title!("Kanji");
    match choose(&[
        "List",
        "Try to guess kanji by meaning",
        "Try to guess kanji by draw",
        "Return to menu"
    ]){
        Some(1) => kanji_list(kanjis_choose()),
        Some(2) => kanji_meaning(kanjis_choose()),
        Some(3) => kanji_draw(kanjis_choose()),
        Some(4) => return menu(),
        _ => kanji()
    }
}