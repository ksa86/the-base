use std::io::{self, Write};
use std::process::{Command, Stdio};

const EXIT_PROG: &str = "exit";

fn main() {
    loop {
        print!("blazing_shell> ");

        io::stdout().flush().unwrap();

        // строка из stdin
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err(){
            eprintln!("reading error");
            continue;
        }

        let input = input.trim();

        // если просто enter на вводе
        if input.is_empty() {
            continue;
        }

        if input.contains('|'){
            // branch with pipe
            let mut command_blocks = input.splitn(2, '|');

            let src_cmd_block = command_blocks.next().unwrap().trim();
            let dst_cmd_block = command_blocks.next().unwrap().trim();

            if src_cmd_block.is_empty() || dst_cmd_block.is_empty(){
                eprintln!("wrong pipeline syntax");
                continue;
            }

            let src_cmd = parse_command_block(src_cmd_block);
            let dst_cmd = parse_command_block(dst_cmd_block);

            if src_cmd.is_none() || dst_cmd.is_none() {
                eprintln!("commnds wrong syntax");
                continue;
            }

            let (src_prog, src_args) = src_cmd.unwrap();
            let (dst_prog, dst_args) = dst_cmd.unwrap();

            if src_prog == EXIT_PROG || dst_prog == EXIT_PROG {
                break;
            }

            // run src, stdout -> pipe
            let src_child = Command::new(src_prog)
                .args(&src_args)
                .stdout(Stdio::piped())
                .spawn();

            let mut src_child = match src_child {
                Ok(child) => child,
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        eprintln!("src_prog '{}' not found", src_prog);
                    } else {
                        eprintln!("can't run '{}': {}", src_prog, e);
                    }
                    continue;
                }
            };

            // stdout src -> stdin dst + run dst
            let src_stdout = src_child.stdout.take().unwrap();
            let dst_child = Command::new(dst_prog)
                .args(&dst_args)
                .stdin(Stdio::from(src_stdout))
                .spawn();

            let mut dst_child = match dst_child {
                Ok(child) => child,
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        eprintln!("dst_prog '{}' not found", dst_prog);
                    } else {
                        eprintln!("can't run '{}': {}", dst_prog, e);
                    }
                    let _ = src_child.kill();
                    continue;
                }
            };

            let _ = src_child.wait();
            let _ = dst_child.wait();
        } else {
            let (command, args) = match parse_command_block(input) {
                Some(res) => res,
                None => continue,
            };

            // выход
            if command == EXIT_PROG {
                break;
            }

            // run
            match Command::new(command).args(&args).spawn() {
                Ok(mut child) => {
                    // запустился...ждём
                    match child.wait() {
                        Ok(status) => {
                            if !status.success() {
                                eprintln!("exit status: {}", status);
                            }
                        }
                        Err(e) => {
                            eprintln!("waiting error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::NotFound {
                        eprintln!("command '{}' not found", command);
                    } else {
                        eprintln!("can't run '{}': {}", command, e);
                    }
                }
            }
        }
    }
}

// возвращает блок (команда + аргументы)
fn parse_command_block(input: &str) -> Option<(&str, Vec<&str>)> {
    let mut parts = input.split_whitespace();
    let command = parts.next()?;
    let args: Vec<&str> = parts.collect();
    Some((command, args))
}
