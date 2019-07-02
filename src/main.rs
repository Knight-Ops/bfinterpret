use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn interpret_bf(bf: &str) {
    let mut instruction_pointer = 0;

    let mut data_buffer: Vec<u8> = vec![0; 30_000];
    let mut data_pointer = 0;

    let mut input_buffer = vec![0; 1];
    let mut loop_counter = 0;

    let bf_vec = bf.as_bytes();

    while instruction_pointer < bf.len() {
        match bf_vec[instruction_pointer] {
            b'>' => {
                if data_pointer == 29_999 {
                    data_pointer = 0
                } else {
                    data_pointer += 1;
                }
                //println!("Data pointer is : {}", data_pointer)
            }
            b'<' => {
                if data_pointer == 0 {
                    data_pointer = 29_999
                } else {
                    data_pointer -= 1;
                }
                //println!("Data pointer is : {}", data_pointer)
            }
            b'+' => {
                if data_buffer[data_pointer] == 0xFF {
                    data_buffer[data_pointer] = 0;
                } else {
                    data_buffer[data_pointer] += 1
                }
            }
            b'-' => {
                if data_buffer[data_pointer] == 0 {
                    data_buffer[data_pointer] = 0xFF;
                } else {
                    data_buffer[data_pointer] -= 1
                }
            }
            b'.' => print!("{}", data_buffer[data_pointer] as char),
            b',' => {
                io::stdin()
                    .read(&mut input_buffer)
                    .expect("Error reading single byte from STDIN");
            }
            b'[' => {
                if data_buffer[data_pointer] == 0 {
                    loop_counter += 1;

                    while instruction_pointer < bf.len() {
                        instruction_pointer += 1;
                        if bf_vec[instruction_pointer] == b'[' {
                            loop_counter += 1;
                        } else if bf_vec[instruction_pointer] == b']' {
                            loop_counter -= 1;

                            if loop_counter == 0 {
                                // instruction_pointer += 1;
                                break;
                            }
                        }
                    }
                }
            }
            b']' => {
                // println!("Loop back if {} nonzero", data_buffer[data_pointer]);
                if data_buffer[data_pointer] != 0 {
                    loop_counter += 1;

                    while instruction_pointer < bf.len() {
                        // println!("Entry is : {}", bf_vec[instruction_pointer]);
                        instruction_pointer -= 1;
                        if bf_vec[instruction_pointer] == b']' {
                            loop_counter += 1;
                        } else if bf_vec[instruction_pointer] == b'[' {
                            loop_counter -= 1;

                            if loop_counter == 0 {
                                // instruction_pointer += 1;
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        instruction_pointer += 1;
        // println!("IP : {}", instruction_pointer);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage : {} [bf program to run]", &args[0]);
        return;
    }

    let bf_program =
        fs::read_to_string(&args[1]).expect("Error while reading provided BF program!");

    interpret_bf(&bf_program);
    // println!("{}", bf_program);
}
