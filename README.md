# 👾 Tamagotchi System Monitor (Rust)

**Tamagotchi System Monitor** is a fun CLI tool that turns your CPU monitoring into a digital pet simulator. Your system's load directly affects your Tamagotchi's health and mood.

## ✨ Features

  * **Real-time Status**: Your pet's face and "mood" react to CPU usage.
  * **HP System**: High CPU load (\>90%) damages your pet. If HP hits 0, it's game over\!
  * **Localization**: English by default, Russian (--lang ru) and Japanese (--lang jp) via flag.

## 🚀 Installation

To install this as a global system utility:

1.  Ensure you have [Rust](https://www.google.com/search?q=https://rustup.rs/) installed.
2.  Clone this repository.
3.  Inside the project folder, run:
    ```bash
    cargo install --path .
    ```

## 🎮 Usage

Once installed, run it from anywhere in your terminal:

```bash
tamagotchi-monitor --lang jp --gentoo
```

### Available Flags:

  * `--lang ru/jp` — Switch to Russian/Japanese language.
  * `--gentoo` — Enable the Gentoo "compiling" mode.


<a name="russian"></a>
# 👾 Tamagotchi System Monitor (Rust)

**Tamagotchi System Monitor** — это CLI-утилита, которая превращает мониторинг процессора в игру. Твой компьютер — это среда обитания питомца. Если процессор перегружен, Тамагочи страдает. Если система отдыхает — он спит и лечится.

[English version below](https://www.google.com/search?q=%23english)

## ✨ Особенности

  * **Состояние в реальном времени**: Лицо питомца и его «настроение» меняются в зависимости от нагрузки на CPU.
  * **Система выживания (HP)**: При нагрузке \>90% питомец теряет здоровье. Если HP упадет до 0 — игра окончена.
  * **Локализация**: Поддержка флага `--lang ru` и `--lang jp`.

## 🚀 Установка

Чтобы установить программу как системную утилиту (чтобы она работала из любого места в терминале):

1.  Убедись, что у тебя установлен [Rust](https://www.google.com/search?q=https://rustup.rs/).
2.  Скачай/склонируй этот репозиторий.
3.  В папке проекта выполни:
    ```bash
    cargo install --path .
    ```

## 🎮 Использование

Теперь ты можешь запустить программу командой (имя бинарника совпадает с `name` в твоем `Cargo.toml`):

```bash
tamagotchi-monitor --lang ru --gentoo
```

### Доступные флаги:

  * `--lang ru/jp` — Включить русский/японский интерфейс.
  * `--gentoo` — Включить сочувствие при компиляции тяжелых пакетов.
