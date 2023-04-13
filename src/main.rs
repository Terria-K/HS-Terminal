mod level;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let question = requestty::Question::select("menu")
        .message("Play Henry Stickmin?")
        .choices(vec![
            "Play",
            "Continue",
            "Quit"
        ])
        .default_separator()
        .build();

    let prompt = requestty::prompt_one(question)?;
    if let Some(prompt) = prompt.as_list_item() {
        match prompt.text.as_str() {
            "Play" => {
                let question = requestty::Question::select("menu")
                    .message("What chapter you want to play?")
                    .choices(vec![
                        "Breaking the Bank",
                        "Escaping the Prison",
                        "Stealing the Diamond"
                    ])
                    .default_separator()
                    .build();
                let prompt = requestty::prompt_one(question)?;
                if let Some(prompt) = prompt.as_list_item() {
                    match prompt.index {
                        0 => level::play_level("levels/btb.json")?,
                        1 => level::play_level("levels/etp.json")?,
                        2 => level::play_level("levels/std.json")?,
                        _ => {}
                    }
                }
            },
            "Continue" => {},
            "Quit" => { std::process::exit(0); },
            _ => {}
        }
    }

    Ok(())
}

