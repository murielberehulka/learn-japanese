mod kanjis;

fn main() {
    //frequency
    let frequency_by_char: Vec<char> = include_str!("kanjis/frequency.txt").chars().collect();
    let mut frequency_by_id: Vec<usize> = Vec::with_capacity(frequency_by_char.len());
    for _ in 0..frequency_by_char.len() {
        frequency_by_id.push(0);
    }
    //kanjis
    let lines = include_str!("kanjis/kanjis.txt").split("\r\n");
    let mut kanjis_str = String::new();
    let mut kanjis_length = 0;
    for line in lines {
        let args: Vec<&str> = line.split('|').collect();
        let kanji = args[1].chars().next().unwrap();
        let strokes = args[2].parse::<u8>().unwrap();
        let meanings = args[3].replace(',',"\",\"");
        let on_yomis = args[4].replace(',',"\",\"");
        let kun_yomis = args[5].replace(',',"\",\"");
        kanjis_str.push_str(&format!(
            "\tKanji {{\n\t\tid: {},\n\t\tkanji: \'{}\',\n\t\tstrokes: {},\n\t\tmeanings: &[\"{}\"],\n\t\ton_yomis: &[\"{}\"],\n\t\tkun_yomis: &[\"{}\"]\n\t}},\n",
            kanjis_length, kanji, strokes, meanings, on_yomis, kun_yomis
        ));
        //find frequency
        let mut freq_rank_pos: usize = 0;
        for freq in &frequency_by_char {
            if kanji == *freq {
                frequency_by_id[freq_rank_pos] = kanjis_length;
                break
            }
            freq_rank_pos += 1;
        }
        kanjis_length += 1;
    }
    let res = format!(
        "use super::Kanji;\n#[allow(dead_code)]\npub const KANJIS: [Kanji;{}] = [\n{}];\n#[allow(dead_code)]\npub const FREQUENCY: [usize;{}] = {:?};",
        kanjis_length, kanjis_str, frequency_by_id.len(), frequency_by_id
    ).to_string();
    std::fs::write("src/kanjis/compiled.rs", res).unwrap();
}