use std::io::Write;
use super::*;

pub fn read() -> String {
    let mut res = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut res).unwrap();
    res.replace('\r',"").replace('\n', "").to_lowercase()
}
pub fn read_number() -> Option<usize> {
    match read().parse::<usize>() {
        Ok(v) => Some(v),
        Err(_) => {
            printlnc!("You must enter an number !", RED);
            press_enter_to_continue!();
            return None
        }
    }
}

pub fn choose(options: &[&str]) -> Option<usize> {
    printlnc!("Options:\n", YELLOW);
    let mut i = 1;
    for option in options {
        printc!(format!("{}", i), YELLOW);
        printlnc!(format!("- {}", option), WHITE);
        i += 1;
    }
    print!("\nChoose an option: ");
    let res = match read_number() {
        Some(v) => v,
        None => return None
    };
    if res > options.len() || res == 0 {
        printlnc!(format!("Option \"{}\" not found !", res), RED);
        press_enter_to_continue!();
        return None
    }
    Some(res)
}

pub fn print_kanjis(kanjis: &[Kanji]) {
    clear!();
    let mut i = 1;
    printlnc!("KANJI  STROKES  MEANING                       ON'YOMIS                      KUN'YOMIS", YELLOW);
    for kanji in kanjis {
        let meanings = kanji.meanings.join(", ");
        let on_yomis = kanji.on_yomis.join(", ");
        let kun_yomis = kanji.kun_yomis.join(", ");
        printc!(format!("{: <6}", kanji.kanji), GREEN);
        printc!(format!("{: <9}", kanji.strokes), WHITE);
        printc!(format!("{: <30}", meanings), WHITE);
        printc!(format!("{: <30}", on_yomis), WHITE);
        printlnc!(format!("{: <30}", kun_yomis), WHITE);
        if i > 20 {
            continue_or_return!();
            i = 0;
        }
        i += 1;
    }
    press_enter_to_continue!();
}

pub fn sort_kanjis_by_frequency(kanjis: &[Kanji]) -> Vec<Kanji> {
    let mut res: Vec<Kanji> = Vec::with_capacity(kanjis.len());
    for kanji_rank_id in kanjis::compiled::FREQUENCY_RANK {
        //find rank kanji in provided kanjis
        for kanji in kanjis {
            if kanji.id == kanji_rank_id {
                res.push(*kanji);
            }
        }
    }
    res
}
