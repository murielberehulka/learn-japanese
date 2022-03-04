use super::*;

pub fn kanjis_choose() -> &'static [Kanji] {
    print_title!("Choose kanjis");
    match choose(&[
        "All kanjis",
        "First grade",
        "Second grade",
        "Thrid grade",
        "Fourth grade",
        "Fifth grade",
        "Sixth grade",
        "List all kanjis used as parts of names of prefectures"
    ]){
        Some(1) => &KANJIS,
        Some(2) => &KANJIS.grade_1(),
        Some(3) => &KANJIS.grade_2(),
        Some(4) => &KANJIS.grade_3(),
        Some(5) => &KANJIS.grade_4(),
        Some(6) => &KANJIS.grade_5(),
        Some(7) => &KANJIS.grade_6(),
        Some(8) => &KANJIS.characters_used_as_parts_of_names_of_prefectures(),
        _ => return kanjis_choose()
    }
}