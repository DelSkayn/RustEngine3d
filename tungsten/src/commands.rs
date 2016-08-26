use super::Terminal;
use super::Console;
use super::Assets;
use super::state::State;

use std::process::Command as StdCommand;


pub fn add_commands<T: Terminal>(c: &mut Console<T>){
    c.add_command("quit", |_, t| {
        t.write("quiting!".to_string());
        State::quit();
    });
    c.add_command("panic", |_,_|{
        panic!();
    });
    c.add_command("asset",asset_command);
    c.add_command("!",sys_command);
}

pub fn sys_command<T: Terminal>(args: &[&str],term: &mut T){
    if args.len() < 1{
        term.write("missing arguments!: ! \"command\"".to_string());
        return
    }
    // TODO defer later
    let mut comm = StdCommand::new(args[0]);
    for i in 1..args.len(){
        comm.arg(args[i]);
    }
    println!("{:?}",comm);

    match comm.spawn(){
        Ok(mut x) => 
        {
            match x.wait(){
                Ok(_) => {},
                Err(e) => term.write(format!("Process recieved error: {:?}",e)),
            }
        },
        Err(e) => {
            term.write(format!("Process could not execute: {:?}",e));
        },

    };
}

fn asset_command<T: Terminal>(args: &[&str],term: &mut T){
    if args.len() < 1 {
        term.write("missing arguments!: asset [[SUBCOMMAND]]".to_string());
        return;
    }
    match args[0]{
        "load" => load_command(&args[1..],term),
        "unload" => unload_command(&args[1..],term),
        _ => term.write(format!("unkown subcommand: {} != [load,unload]",args[0])),
    }
}

fn load_command<T: Terminal>(args: &[&str], term: &mut T){
    match args[0]{
        "mesh" => {
            if args.len() < 3{
                term.write("missing arguments! load [TYPE] [NAME] [PATH]".to_string());
                return;
            }
            Assets::load_mesh(args[1].to_string(),args[2]);
            term.write("mesh loaded".to_string());
        }
        _ => term.write(format!("Wrong type: {} != [mesh]",args[0])),
    }
}

fn unload_command<T: Terminal>(args: &[&str], term: &mut T){
    match args[0]{
        "mesh" => {
            if args.len() < 2{
                term.write("missing arguments: unload [TYPE] [NAME]".to_string());
                return;
            }
            Assets::unload_mesh(&args[1].to_string());
            term.write("mesh unloaded".to_string());
        }
        _ => term.write(format!("Wrong type: {} != [mesh]",args[0])),
    }
}
