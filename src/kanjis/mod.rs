pub mod compiled;

#[derive(Copy, Clone)]
pub struct Kanji {
    pub id: usize,
    pub kanji: char,
    pub strokes: u8,
    pub meanings: &'static [&'static str],
    pub on_yomis: &'static [&'static str],
    pub kun_yomis: &'static [&'static str]
}

pub trait Kanjis {
    fn grade_1(&self) -> &[Kanji];
    fn grade_2(&self) -> &[Kanji];
    fn grade_3(&self) -> &[Kanji];
    fn grade_4(&self) -> &[Kanji];
    fn grade_5(&self) -> &[Kanji];
    fn grade_6(&self) -> &[Kanji];
    fn characters_used_as_parts_of_names_of_prefectures(&self) -> &[Kanji];
}
impl Kanjis for [Kanji] {
    fn grade_1(&self) -> &[Kanji] {
        &self[0..80]
    }
    fn grade_2(&self) -> &[Kanji] {
        &self[81..240]
    }
    fn grade_3(&self) -> &[Kanji] {
        &self[241..440]
    }
    fn grade_4(&self) -> &[Kanji] {
        &self[441..640]
    }
    fn grade_5(&self) -> &[Kanji] {
        &self[641..825]
    }
    fn grade_6(&self) -> &[Kanji] {
        &self[826..1006]
    }
    fn characters_used_as_parts_of_names_of_prefectures(&self) -> &[Kanji] {
        &self[1007..1026]
    }
}