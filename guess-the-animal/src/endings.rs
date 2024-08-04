use crate::file_utils::Line;
use anyhow::Result;

pub fn animal_guess(animal: String) -> Result<()> {
    let is_right = crate::utils::ask_yes_or_no(format!("Is the animal '{}'?", animal))?;

    if !is_right {  idk()?  }

    println!("Yay, got it right :-)");

    Ok(())
}

pub fn idk() -> Result<()> {
    println!("You win!");

    let line = Line::new();

    println!("What was the animal?");
    std::io::stdin().read_line(&mut line.animal)?;
    line.animal.pop()?;

    println!("What question would you ask to differentiate?");
    std::io::stdin().read_line(&mut line.question)?;
    line.question.pop()?;

    crate::file_utils::write_line_to_file(line);
    Ok(())
}