use super::*;
use rand::Rng;

pub fn kanji_meaning(kanjis: &[Kanji]) {
    print_title!("Try to guess kanji meaning");

    //choose wrong kanjis
    let mut wrong_kanji_ids: Vec<usize> = Vec::with_capacity(OPTIONS);
    for _ in 0..OPTIONS {
        let mut wrong_id = 0;
        //test if kanji is already choosed
        let mut exists = true;
        while exists {
            wrong_id = rand::thread_rng().gen_range(0..kanjis.len());
            exists = false;
            for test_wrong_kanji_id in &wrong_kanji_ids {
                if *test_wrong_kanji_id == wrong_id {
                    exists = true;
                }
            }
        }
        //insert kanji in the wrong kanjis vector
        wrong_kanji_ids.push(wrong_id);
    }

    //set one of the wrong kanjis to be the right choice
    let right_kanji_vec_id: usize = rand::thread_rng().gen_range(0..OPTIONS);
    let right_kanji = &kanjis[wrong_kanji_ids[right_kanji_vec_id]];
    printc!("The kanji \"", WHITE);
    printc!(format!("{}", right_kanji.kanji), GREEN);
    printlnc!("\" means:\n", WHITE);
    let kanji_ids = wrong_kanji_ids;

    //print all kanjis
    let mut i = 0;
    for kanji_id in kanji_ids {
        let kanji = &kanjis[kanji_id];
        printc!(format!("{} - ", i + 1), YELLOW);
        printlnc!(format!("{}", kanji.meanings.join(", ")), GREEN);
        i += 1;
    }
    
    //get the answer
    printc!("\nAnswer (or C to return): ", WHITE);
    let res = read();
    if res == "c" {
        return sections::menu();
    }
    let res_parsed = match res.parse::<usize>() {
        Ok(v) => v,
        Err(_) => {
            printlnc!("You must enter an number !", RED);
            press_enter_to_continue!();
            return kanji_meaning(kanjis);
        }
    };
    if res_parsed == right_kanji_vec_id + 1 {
        //right choice
        printlnc!("\nWright answer !", GREEN);
    }else {
        //wrong choice
        printlnc!("\nWrong answer !", RED);
        printc!("The right answer is: ", WHITE);
        printlnc!(format!(" {} ", right_kanji.meanings.join(", ")), GREEN);
    }
    press_enter_to_continue!();
    kanji_meaning(kanjis);
}