use super::Terminal;
use asset::Assets;

pub fn asset_command<T: Terminal>(args: &[&str],term: &mut T){
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
            Assets::unload_mesh(args[1].to_string());
            term.write("mesh unloaded".to_string());
        }
        _ => term.write(format!("Wrong type: {} != [mesh]",args[0])),
    }
}
