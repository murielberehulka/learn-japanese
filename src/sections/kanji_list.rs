use super::*;

pub fn kanji_list(kanjis: &[Kanji]) {
    match kanjis_sort(kanjis) {
        Some(sorted) => print_kanjis(&sorted),
        None => print_kanjis(kanjis)
    }
    menu();
}