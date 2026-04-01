use colored::*;
use std::env;
use std::io::{self, Write};
use std::thread;
use sysinfo::System;

#[derive(Clone, Copy)]
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
    err_read: &'static str,
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
                err_read: "Ошибка: не удалось прочитать строку!",
                nm: "Имя:",
                gentoo_lang: "(Ты там @world компилируешь?)",
                mood_sleeping: "спит...",
                mood_ok: "в норме",
                mood_hard: "тяжело...",
                mood_help: "ПОМОГИТЕ",
            },
            Language::English => Self {
                msg_death: "SYSTEM OVERHEAT. TAMAGOCHI IS DEAD.",
                msg_cpu: "CPU",
                msg_mood: "Mood",
                hi: "Hi! It's tamagochi-monitor!",
                call: "Name your tamagochi: ",
                err_read: "Error: failed to read line!",
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
                call: "たまごっちの名前を決めてね：",
                err_read: "エラー：行の読み込みに失敗しました！",
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

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

fn main() {
    let mut sys = System::new_all();
    let mut hp: i32 = 100;
    let args: Vec<String> = env::args().collect();

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
    let is_gentoo = args.iter().any(|arg| arg == "--gentoo");
    let gentoo_suffix = if is_gentoo {
        format!(" {}", msgs.gentoo_lang.italic().dimmed())
    } else {
        "".to_string()
    };

    println!("{}", msgs.hi.bright_cyan().bold());
    print!("{}", msgs.call.yellow());
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect(msgs.err_read);
    let name = name.trim();

    loop {
        sys.refresh_cpu_all();
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let usage = sys.global_cpu_usage();

        if usage > 90.0 {
            hp = (hp - 2).max(0);
        } else if usage < 20.0 {
            hp = (hp + 1).min(100);
        }

        clear_screen();

        if hp <= 0 {
            println!(
                "{} {} {}",
                "( x _ x )".red(),
                msgs.msg_death.red().bold(),
                gentoo_suffix
            );
            break;
        }

        let (face, mood) = match usage {
            u if u < 10.0 => ("( - _ -) zzz", msgs.mood_sleeping.blue()),
            u if u <= 50.0 => ("( o . o )", msgs.mood_ok.green()),
            u if u <= 90.0 => ("( > ﹏ < )", msgs.mood_hard.yellow()),
            _ => ("( x _ x )", msgs.mood_help.red().blink()),
        };

        let border = "------------------------------";
        if usage > 90.0 {
            println!("{}", border.red());
            println!("!! {:^24} !!", face.red());
            println!("{}", border.red());
        } else {
            println!("{}", border.cyan());
            println!("|  {:^24}  |", face.cyan());
            println!("{}", border.cyan());
        }

        let filled = (hp / 10).max(0) as usize;
        let empty = 10 - filled;
        let bar = format!("[{}{}]", "#".repeat(filled), "-".repeat(empty));

        let bar_colored = match hp {
            h if h > 70 => bar.green(),
            h if h > 30 => bar.yellow(),
            _ => bar.red(),
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
