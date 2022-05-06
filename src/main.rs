use midir::{MidiOutput};

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::stdin;
use std::process;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

struct Commands {
    commands: HashMap<String, CommandInfo>
}

struct CommandInfo {
    handler: Box<dyn Fn(Vec<&str>) -> [u8; 3]>,
    help: String
}

impl Commands {
    fn new<'a>() -> Commands {
        Commands {
            commands: HashMap::new()
        }
    }
    
    fn add(&mut self, name: String, handler: Box<dyn Fn(Vec<&str>) -> [u8; 3]>, help: String) {
        self.commands.insert(name, CommandInfo {handler: handler, help: help});
    }

    fn run(&mut self, name: String, parts: Vec<&str>) -> Option<[u8; 3]> {
        match self.commands.get(&name) {
            Some(command_info) => {
                Some((command_info.handler)(parts))
            }
            None => None
        }
    }
}

fn noteon_message(note: u8, velocity: u8, channel: u8) -> [u8; 3] {
    [0x90 | channel, note, velocity]
}

fn handle_noteon_command(parts: Vec<&str>) -> [u8; 3] {
    noteon_message(parts[1].parse().unwrap(), parts[2].parse().unwrap(), parts[3].parse().unwrap())
}

fn cc_message(control: u8, value: u8, channel: u8) -> [u8; 3] {
    [0xB0 | channel, control, value]
}

fn handle_cc_command(parts: Vec<&str>) -> [u8; 3] {
    cc_message(parts[1].parse().unwrap(), parts[2].parse().unwrap(), parts[3].parse().unwrap())
}

fn run() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("midiline")?;
    
    let out_ports = &midi_out.ports();

    if env::args().len() == 1 {
        println!("MIDI output ports");
        for (i, p) in out_ports.iter().enumerate() {
            println!("  {}: '{}'", i, midi_out.port_name(&p).unwrap());
        }

        process::exit(0);
    }

    let mut port_idx: Option<usize> = None;
    
    for (i, p) in out_ports.iter().enumerate() {
        if midi_out.port_name(&p).unwrap() == env::args().nth(1).unwrap() {
            port_idx = Some(i);
            break;
        }
    }

    let mut conn_out = midi_out.connect(&out_ports[port_idx.unwrap()], "midiline")?;
    let mut commands = Commands::new();
    
    commands.add("noteon".to_string(), Box::new(handle_noteon_command), "noteon NOTE VELOCITY CHANNEL".to_string());
    commands.add("cc".to_string(), Box::new(handle_cc_command), "cc CONTROL VALUE CHANNEL".to_string());
    

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let parts: Vec<&str> = input.trim().split(" ").collect();

        match parts[0] {
            "quit" => {
                break;
            }
            "help" => {
                for (name, info) in commands.commands.iter() {
                    println!("'{}': {}", name, info.help)
                }
            }
            _ => {
                match commands.run(String::from(parts[0]), parts) {
                    Some(bs) => {
                        conn_out.send(&bs).unwrap();
                    }
                    None => {}
                }
            }
        }
    }

    Ok(())
}
