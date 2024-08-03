use std::fs::File;

const FILE_PATH = ".database";

struct Line {
    question: mut String,
    animal: mut String,
    n: u32,
}

fn get_line_struct(line: String) -> Line {
}

fn get_line_from_file(n: u32) -> Result<Option<Line>> {
    let file = File::open(FILE_PATH)?;
    let reader = new BufReader::new(file);
    for (index, line) in reader.lines.enumerate() {
        if index == n {
            Ok(Some(get_line_struct(line)))
        }
    }
    Ok(None)
}

fn write_line_to_file(line: Line) {
}

fn ask_yes_or_no(s: String) -> bool {
    print!("{}\t(y/n):\t", s);
    /* read input
    match answer.to_upper() {
        'Y' => true,
        'N' => false,
        _ => ask_yes_or_no(s),
    } */
}

fn end(animal: String) -> Result<()> {
    let is_right = ask_yes_or_no(format!("Is the animal '{}'?", animal));

    println!("You win!");

    let line = Line::new();

    println!("What was the animal?");
    std::io::stdin().read_line(&mut line.animal)?;
    line.animal.pop()?;

    println!("What question would you ask to differentiate?");
    std::io::stdin().read_line(&mut line.question)?;
    line.question.pop()?;

    write_line_to_file(line);
    Ok(())
}

fn ask(line: Line) -> Result<String> {
    let answer = ask_yes_or_no(line.question);
    match answer {
        true => {
            match get_line_from_file(line.n)? {
                None => end(line.animal)?;
                Some(line) => ask(line)
            }
        },
        false => Ok(line.animal)
    }
}

fn game_start() -> Result<()> {
    match get_line_from_file(i)? {
        None => {},
        Some(question) => {
            ask(question)?;
        },
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("think of an animal\n");
    
    game_start()?;

    Ok(())
}
