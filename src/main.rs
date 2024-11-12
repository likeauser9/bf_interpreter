use std::{env, fs::File, io::{self, Read, Write}};

fn run_brainfuck(program: &str) {
    let mut memory = vec![0u8; 30000];
    let mut pointer = 0;
    let mut pc= 0;
    let program_bytes = program.as_bytes();

    while pc < program_bytes.len() {
        match program_bytes[pc] {
            b'>' => pointer += 1,
            b'<' => pointer -= 1,
            b'+' => memory[pointer] = memory[pointer].wrapping_add(1),
            b'-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            b'.' => {
                print!("{}", memory[pointer] as char);
                io::stdout().flush().unwrap();
            },
            b',' => {
                memory[pointer] = match io::stdin().bytes().next() {
                    Some(Ok(byte)) => byte,
                    _ => 0,
                };
            }
            b'[' => {
                if memory[pointer] == 0 {
                    let mut open_brackets = 1;
                    while open_brackets > 0 {
                        pc += 1;
                        if program_bytes[pc] == b'[' {
                            open_brackets += 1;
                        } else if program_bytes[pc] == b']' {
                            open_brackets -= 1;
                        }
                    }
                }
            },
            b']' => {
                if memory[pointer] != 0{
                    let mut close_brackets = 1;
                    while close_brackets > 0 {
                        pc -= 1;
                        if program_bytes[pc] == b']' {
                            close_brackets += 1;
                        } else if program_bytes[pc] == b'[' {
                            close_brackets -= 1;
                        }
                    }
                }
            },
            _ => {}
        }
        pc += 1;
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: brainfuck <input.bf> -o <output>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[3];

    let mut file = File::open(input_file)?;
    let mut program = String::new();

    file.read_to_string(&mut program)?;

    run_brainfuck(&program);

    let mut output = File::create(output_file)?;
    write!(output, "Program executed successfully")?;

    Ok(())
}

