use super::*;

pub fn kanjis_sort(kanjis: &[Kanji]) -> Option<Vec<Kanji>> {
    print_title!("Sort kanjis");
    match choose(&[
        "All kanjis",
        "Most common used"
    ]){
        Some(1) => None,
        Some(2) => Some(sort_kanjis_by_frequency(kanjis)),
        _ => return kanjis_sort(kanjis)
    }
}