use std::io::Write;

pub fn read_line(prompt: Option<&'static str>) -> std::io::Result<String> {
    match prompt {
        Some(p) => {
            print!("{}", p);
            std::io::stdout().flush()?;
        }
        None => (),
    };
    let mut res = String::new();
    let _ = std::io::stdin().read_line(&mut res)?;
    return Ok(res.trim().to_string());
}

pub fn yn_to_bool(yn: &str) -> Option<bool> {
    match yn {
        "y" => Some(true),
        "n" => Some(false),
        _ => None,
    }
}
