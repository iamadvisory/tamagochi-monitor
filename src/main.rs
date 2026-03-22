use std::env;
use std::io::{self, Write};
use std::thread;
use sysinfo::System;

fn main() {
    // sysinfo
    let mut sys = System::new_all();

    // hp
    let mut hp = 100;

    let args: Vec<String> = env::args().collect();

    // ru flag
    let is_russian = args.contains(&"--lang".to_string()) && args.contains(&"ru".to_string());

    let is_gen = args.contains(&"--gentoo".to_string());

    // text translation
    let msg_death = if is_russian {
        "СИСТЕМА ПЕРЕГРЕТА. ТАМАГОЧИ ПОГИБ."
    } else {
        "SYSTEM OVERHEAT. TAMAGOCHI IS DEAD."
    };
    let msg_cpu = if is_russian {
        "Загрузка"
    } else {
        "CPU"
    };
    let msg_mood = if is_russian {
        "Состояние"
    } else {
        "Mood"
    };
    let hi = if is_russian {
        "Привет! Это тамагочи-монитор!"
    } else {
        "Hi! It's tamagochi-monitor!"
    };
    let call = if is_russian {
        "Назови своего тамагочи: "
    } else {
        "Call your tamagochi: "
    };
    let errrl = if is_russian {
        "Error: не получилось прочитать строку!!!"
    } else {
        "Error: failed to read line!!!"
    };
    let nm = if is_russian { "Имя: " } else { "Name: " };
    let gentoo_lang = if is_russian {
        "(Ты там @world компилируешь)?"
    } else {
        "(Are you compiling @world there?)"
    };

    let gentoo = if is_gen { gentoo_lang } else { "" };

    println!("{hi}");

    println!("{call}");

    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect(&errrl);
    let name = input_name.trim();

    loop {
        // Clear
        print!("\x1B[2J\x1B[H");

        // sysinfo
        sys.refresh_cpu_all();
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        let usage = sys.global_cpu_usage();

        // hp
        if usage > 90.0 {
            hp -= 2;
        } else if usage < 20.0 && hp < 100 {
            hp += 1;
        }

        // death
        if hp <= 0 {
            print!("\x1B[2J\x1B[H");
            println!("( x _ x ) {msg_death} {gentoo}");
            break;
        }

        // face
        let (face, mood) = match usage {
            u if u < 10.0 => (
                "( - _ -) zzz",
                if is_russian {
                    "сон..."
                } else {
                    "sleeping..."
                },
            ),
            u if u <= 50.0 => (
                "( o . o )",
                if is_russian {
                    "ладно..."
                } else {
                    "it's ok..."
                },
            ),
            u if u <= 90.0 => (
                "( > ﹏ < )",
                if is_russian {
                    "тяжело..."
                } else {
                    "it's hard..."
                },
            ),
            _ => (
                "( x _ x )",
                if is_russian {
                    "ПОМОГИТЕ"
                } else {
                    "HELP ME"
                },
            ),
        };

        // home
        if usage > 90.0 {
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
            println!("!!      {face}      !!");
            println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        } else {
            println!("------------------------------");
            println!("|      {face}      |");
            println!("------------------------------");
        }

        // bar format
        let filled = (hp / 10).max(0) as usize;
        let empty = 10 - filled;
        let bar = format!("[{}{}]", "#".repeat(filled), "-".repeat(empty));

        // hp bar
        println!("HP: {bar} {hp}/100");

        println!("{nm} {name}");

        // CPU usage
        println!("{msg_cpu}: {usage:.1}% | {msg_mood}: {mood}");

        io::stdout().flush().unwrap();
    }
}
