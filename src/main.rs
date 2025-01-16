use colored::Colorize;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let words = text_to_words(read_text());
    quiz(&words)
}

fn quiz(words: &Vec<Word>) {
    let mut words = words.clone();

    loop {
        // 출제
        let (selected, i) = {
            if words.is_empty() {
                break;
            }
            let i = thread_rng().gen_range(0..words.len());
            let selected: &Word = words.get(i).unwrap();
            print!("{}:\n>>> ", selected.word);
            stdout().flush().unwrap();
            (selected, i)
        };

        // 줄 읽기
        let responds = {
            let mut s = String::new();
            stdin().read_line(&mut s).unwrap();
            s.trim()
                .split(',')
                .map(|x| x.trim().to_owned())
                .collect::<Vec<String>>()
        };

        let mut do_stop = false;
        let mut is_answer_correct = true;

        // todo (괄호) 내용을 입력 안 했을 경우 노란색으로 띄워주고 넘어가기
        // todo 단어 수정/추가 방법 넣기

        let meanings = {
            let mut meanings = selected
                .meanings
                .iter()
                .map(|(x, y)| (false, x, y))
                .collect::<Vec<(bool, &String, &String)>>();

            let print = responds.iter().map(|res_raw| {
                let res = res_raw.replace(' ', "");

                if res == "stop" {
                    do_stop = true;
                    String::new()
                } else if let Some((is_answered, answer_raw, _)) =
                    meanings.iter_mut().find(|(_, _, answer)| **answer == res)
                {
                    *is_answered = true;
                    format!(
                        "{}{}",
                        answer_raw.bright_green(),
                        if res_raw != *answer_raw { format!("({})", res_raw).bright_black()} else { "".bright_black() }
                    )
                } else {
                    is_answer_correct = false;
                    res_raw.bright_red().to_string()
                }
            }).collect::<Vec<String>>().join(", ");

            // 전부 맞춰야 정답 처리
            if !meanings.iter().all(|(is_answered, _, _)| *is_answered) {
                is_answer_correct = false;
            }

            println!(
                "{} {}",
                if is_answer_correct { "[O]".green() } else { "[-]".red() },
                print
            );

            meanings
        };

        // 후처리
        {
            if is_answer_correct {
                words.remove(i);
            } else {
                let message = meanings.iter()
                    .map(
                        |(is_answered, answer_raw, _)|
                            (if *is_answered { answer_raw.bright_green() } else { answer_raw.bright_red() }).to_string()
                    ).collect::<Vec<String>>().join(", ");
                
                println!("correct: {}", message);
            }
            println!();

            if do_stop {
                break;
            }
        }
    }
}

fn read_text() -> String {
    let Ok(mut read) = File::open("words.txt") else {
        println!("{}", "Cannot find words.txt.\n\
            Enter any text to close this window.\n\n\
            words.txt를 찾을 수 없습니다.\n\
            창을 닫으려면 아무 내용이나 입력하십시오. "
            .bright_red());
        let _ = stdin().read_line(&mut String::new());
        panic!()
    };
    let mut buf = String::new();
    read.read_to_string(&mut buf).unwrap();
    buf
}

fn text_to_words(text: String) -> Vec<Word> {
    text.lines()
        .map(|x| {
            let (word, meanings) = x.split_once(':').unwrap();
            Word {
                word: word.trim().to_owned(),
                meanings: meanings
                    .split(',')
                    .map(|x| x.trim().to_owned())
                    .map(|x| (x.clone(), x.replace(' ', "")))
                    .collect(),
            }
        })
        .collect()
}

#[derive(Clone)]
struct Word {
    word: String,
    meanings: Vec<(String, String)>,
}
