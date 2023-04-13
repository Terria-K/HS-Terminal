use std::{error::Error, thread::sleep, time::Duration, collections::HashMap};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Level {
    pub title: String,
    pub introduce: String,
    pub sub_levels: HashMap<String, SubLevel>
}

#[derive(Deserialize)]
pub struct SubLevel {
    pub message: String,
    #[serde(rename = "type")]
    pub choice_type: ChoiceType,
    pub choices: Vec<Choice>
}

#[derive(Deserialize)]
pub enum ChoiceType {
    Single,
    Multiple
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Choice {
    Success {
        name: String,
        route_to: String,
        award: Option<String>
    },
    Failure {
        name: String,
        message: String
    }
}

impl Choice {
    fn to_name_vec(choice: &[Self]) -> Vec<String> {
        choice.iter().map(|x| {
            match x {
                Choice::Success { name, route_to: _, award: _ } => name.to_owned(),
                Choice::Failure { name, message: _ } => name.to_owned(),
            }
        }).collect::<Vec<String>>()
    }
}

pub fn play_level(path: &str) -> Result<(), Box<dyn Error>> {
    let fs = std::fs::read_to_string(path)?;
    let level = serde_json::from_str::<Level>(&fs)?;
    play_ground(level)?;
    Ok(())
}

fn play_sublevel(sub_level: &SubLevel) -> Result<Choice, Box<dyn Error>> {
    let request = requestty::Question::select("select")
        .message(&sub_level.message)
        .choices(Choice::to_name_vec(&sub_level.choices))
        .default_separator()
        .build();
    
    let prompt = requestty::prompt_one(request)?;
    let Some(prompt) = prompt.as_list_item() else { return Err("Not in the list".into()) };
    let choice = sub_level.choices.get(prompt.index);
    let Some(choice) = choice else { return Err("Choices out of bounds".into()) };
    Ok(choice.clone())
}

fn get_level_introduction(level: &Level) -> Result<&SubLevel, String>{
    let introduce = level.introduce.clone();
    let Some(sub) = level.sub_levels.get(&introduce) else {
        return Err("Missing Introduction".into()); 
    };
    Ok(sub)
}

fn play_ground(level: Level) -> Result<(), Box<dyn Error>> {
    println!("{}", level.title);
    let mut sub = get_level_introduction(&level)?;

    // Rust is dumb, this is reachable
    #[allow(unreachable_code)]
    {
    let result = loop {
        println!();
        sleep(Duration::from_millis(2000));

        let sub_level = play_sublevel(sub)?;
        sleep(Duration::from_millis(2000));

        match sub_level {
            Choice::Success { name, award, route_to } => {
                println!("{name} is successful!");
                println!();
                if route_to == "completed" {
                    println!("Chapter Completed!");
                    if let Some(award) = award {
                        println!("You got an award: ");
                        println!("{award}"); 
                    }
                    break Ok(())
                }
                let Some(route) = level.sub_levels.get(&route_to) else {
                    return Err("Route not found!")?;
                };
                sub = route;
            },
            Choice::Failure { name, message } => {
                println!("{name}: You FAILED!");
                let request = requestty::Question::select("failed")
                    .message(message)
                    .choices(vec![
                        "Retry",
                        "Quit"
                    ])
                    .default_separator()
                    .build();
                let prompt = requestty::prompt_one(request)?;
                if let Some(prompt) = prompt.as_list_item() {
                    match prompt.index {
                        0 => {},
                        1 => { break Ok(()) },
                        _ => {}
                    }
                }
            },
        }
    };
    result
    }
}
