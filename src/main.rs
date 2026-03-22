use colored::*;
use std::env;
use std::io::{self, Write};
use std::thread;
use sysinfo::System;

enum Language {
    English,
    Russian,
    Japanese,
}

struct Messages {
    msg_death: &'static str,
    msg_cpu: &'static str,
    msg_mood: &'static str,
    hi: &'static str,
    call: &'static str,
    errrl: &'static str,
    nm: &'static str,
    gentoo_lang: &'static str,
    mood_sleeping: &'static str,
    mood_ok: &'static str,
    mood_hard: &'static str,
    mood_help: &'static str,
}

impl Messages {
    fn new(lang: Language) -> Self {
        match lang {
            Language::Russian => Self {
                msg_death: "СИСТЕМА ПЕРЕГРЕТА. ТАМАГОЧИ ПОГИБ.",
                msg_cpu: "CPU",
                msg_mood: "Состояние",
                hi: "Привет! Это тамагочи-монитор!",
                call: "Назови своего тамагочи: ",
                errrl: "Error: не удалось прочитать строку!!!",
                nm: "Имя:",
                gentoo_lang: "(Ты там @world компилируешь?)",
                mood_sleeping: "сон...",
                mood_ok: "ладно...",
                mood_hard: "тяжело...",
                mood_help: "ПОМОГИТЕ",
            },
            Language::English => Self {
                msg_death: "SYSTEM OVERHEAT. TAMAGOCHI IS DEAD.",
                msg_cpu: "CPU",
                msg_mood: "Mood",
                hi: "Hi! It's tamagochi-monitor!",
                call: "Call your tamagochi: ",
                errrl: "Error: failed to read line!!!",
                nm: "Name:",
                gentoo_lang: "(Are you compiling @world there?)",
                mood_sleeping: "sleeping...",
                mood_ok: "it's ok...",
                mood_hard: "it's hard...",
                mood_help: "HELP ME",
            },
            Language::Japanese => Self {
                msg_death: "システム過熱。たまごっちは力尽きた...",
                msg_cpu: "CPU使用率",
                msg_mood: "気分",
                hi: "こんにちは！たまごっちモニターへようこそ！",
                call: "たまгоっちの名前を決めてね：",
                errrl: "エラー：行の読み込みに失敗しました！！！",
                nm: "名前:",
                gentoo_lang: "(@world をコンパイル中ですか？)",
                mood_sleeping: "眠い...",
                mood_ok: "大丈夫...",
                mood_hard: "苦しい...",
                mood_help: "助けて！！",
            },
        }
    }
}

fn main() {
    let mut sys = System::new_all();
    let mut hp = 100;
    let args: Vec<String> = env::args().collect();

    // Определение языка
    let mut lang_choice = Language::English;
    if let Some(pos) = args.iter().position(|r| r == "--lang") {
        if let Some(value) = args.get(pos + 1) {
            lang_choice = match value.as_str() {
                "ru" => Language::Russian,
                "jp" => Language::Japanese,
                _ => Language::English,
            };
        }
    }

    let msgs = Messages::new(lang_choice);
    let is_gen = args.contains(&"--gentoo".to_string());
    let gentoo = if is_gen {
        msgs.gentoo_lang.italic().dimmed().to_string()
    } else {
        "".to_string()
    };

    println!("{}", msgs.hi.bright_cyan().bold());
    print!("{}", msgs.call.yellow());
    io::stdout().flush().unwrap();

    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect(msgs.errrl);
    let name = input_name.trim();

    loop {
        print!("\x1B[2J\x1B[H");

        sys.refresh_cpu_all();
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let usage = sys.global_cpu_usage();

        if usage > 90.0 {
            hp -= 2;
        } else if usage < 20.0 && hp < 100 {
            hp += 1;
        }

        if hp <= 0 {
            print!("\x1B[2J\x1B[H");
            println!("( x _ x ) {} {}", msgs.msg_death.red().bold(), gentoo);
            break;
        }

        let (face, mood) = match usage {
            u if u < 10.0 => ("( - _ -) zzz", msgs.mood_sleeping.blue()),
            u if u <= 50.0 => ("( o . o )", msgs.mood_ok.green()),
            u if u <= 90.0 => ("( > ﹏ < )", msgs.mood_hard.yellow()),
            _ => ("( x _ x )", msgs.mood_help.red().blink()),
        };

        if usage > 90.0 {
            println!("{}", "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".red());
            println!("!!      {}      !!", face.red());
            println!("{}", "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!".red());
        } else {
            println!("------------------------------");
            println!("|      {}      |", face.cyan());
            println!("------------------------------");
        }

        let filled = (hp / 10).max(0) as usize;
        let empty = 10 - filled;
        let bar_str = format!("[{}{}]", "#".repeat(filled), "-".repeat(empty));
        let bar_colored = if hp > 70 {
            bar_str.green()
        } else if hp > 30 {
            bar_str.yellow()
        } else {
            bar_str.red()
        };

        println!("HP: {} {}/100", bar_colored, hp);
        println!("{} {}", msgs.nm.bold(), name.bright_white());
        println!(
            "{}: {:.1}% | {}: {}",
            msgs.msg_cpu, usage, msgs.msg_mood, mood
        );

        io::stdout().flush().unwrap();
    }
}
