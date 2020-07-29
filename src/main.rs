#[macro_use] extern crate log;

use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
use log::{info, warn};


mod hostsfile;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(),ExitFailure>{
    let args = Cli::from_args();

    info!("Parsing {:?} for \"{}\"",args.path,args.pattern);

    //TODO the whole file is read into memory here
    /*
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("Could not read file {:?}",args.path))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}",line);
        }
    }*/

    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("Could not read file {:?}",args.path))?;

    let hosts = hostsfile::HostsFile::from_string(content.as_str());
    for host in hosts.entries {
        if host.contains_host() {
            println!("{}", match host.ip {
                Some(i) => i,
                None => "No IP".to_string()
            });
        }
    }

    Ok(())
}
