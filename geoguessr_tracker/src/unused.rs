/* fn get_cmd_type(input_string: String) -> Cmd {
    let cmd_string: String = input_string.chars().filter(|c| *c != ',').collect();
    let cmd_char: char = cmd_string.chars().next().unwrap(); //unwrap because validator would have caught it

    match cmd_char {
        'q' => Cmd::Quit,
        's' => Cmd::PrintStats,
        'c' | 'f' => {
            if cmd_string.len() < 3 {
                return Cmd::Invalid { reason: "".into() };
            }
            let mut cmd_str_iter = cmd_string.chars();
            if cmd_str_iter.nth(1).unwrap() != ' ' {
                return Cmd::Invalid { reason: "".into() };
            }

            let mut score_string = String::new();
            for c in cmd_str_iter {
                if !c.is_ascii() {
                    return Cmd::Invalid { reason: "".into() };
                }
                score_string.push(c);
            }

            match cmd_char {
                'c' => Cmd::AddEntryCalculated { score_string },
                'f' => Cmd::FixPrev { score_string },
                _ => unreachable!(),
            }
        },
        'd' => {
            let cmd_explanation = "command `d` should be followed by <space><score><space><total score>";
            if cmd_string.len() < 5 {
                return Cmd::Invalid { reason: cmd_explanation.into() };
            }
            let mut cmd_str_iter = cmd_string.chars();
            if cmd_str_iter.nth(1).unwrap() != ' ' {
                return Cmd::Invalid { reason: cmd_explanation.into() };
            }

            let cmd_str_iter = cmd_str_iter.enumerate();
            let mut score_string = String::new();
            let mut total_score_string = String::new();
            let mut second_score =

            for (i,c) in cmd_str_iter {
                if c.is_ascii() && i < 4 {  // the score for a single round is max 4 chars long
                    score_string.push(c);
                }
                else if c == ' ' {
                    break;
                }
                else {
                    return Cmd::Invalid { reason: cmd_explanation.into() };
                }
            }


            let mut score_str_iter = cmd_string.chars();
            score_str_iter.nth(1);
            let score_string: String = score_str_iter.collect();

        }
        _ => Cmd::Invalid { reason: "".into() },
    }
} */

/* fn known_cmd_char_validator(input: &str) -> Result<Validation, CustomUserError> {
    if let Some(cmd_char) = input.chars().next() {
        match cmd_char {
            'y' | 'n' | 'd' | 'q' => Ok(Validation::Valid),
            char => {
                let mut msg: String = "unkown command char: ".to_string();
                msg.push(char);
                Ok(Validation::Invalid(msg.into()))
            }
        }
    } else {
        Ok(Validation::Invalid(
            "invalid command".into(),
        ))
    }
} */
/* fn len_validator(input: &str) -> Result<Validation, CustomUserError> {
    if input.len() > 2 {
        Ok(Validation::Invalid(
            "commands should given in the form: <char>{Enter}".into(),
        ))
    } else {
        Ok(Validation::Valid)
    }
} */

/* fn handle_cmd_str(cmd_str: &str) -> bool {
    let cmd_char_res = char::from_str(cmd_str.trim_end());
    match cmd_char_res {
        Ok(cmd_char) => {
            match cmd_char {
                'y' => {
                    add_5k();
                    false
                },
                'n' => {
                    add_fail();
                    false
                },
                'd' => {
                    del_remove_entry();
                    false
                },
                'q' => {
                    true
                }
            }
        },
        Err(_err) => {
            println!("invalid command");
            false
        }
    }
} */
