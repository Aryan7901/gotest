use std::env::args;
use std::process::{Command,Stdio, exit};
use std::io::Write;
static GOTEST_SCRIPT:&str=include_str!("../gotest.sh");
fn main() {
   let args:Vec<String>=args().skip(1).collect();
   args_parse(args);
}

fn args_parse(args:Vec<String>){
    let mut non_flags:Vec<String>=Vec::new();
    let mut html_flag="0";
    let mut open_flag="0";
    for arg in args{
        if arg=="-ho"{
            html_flag="1";
            open_flag="1";
        }
        else if arg=="-h"||arg=="--html"{
            html_flag="1";
        }
        else if arg=="-o"||arg=="--open"{
            open_flag="1";
        }
        else{
            non_flags.push(arg);
        }
    }
    let path=if let Some(path)=non_flags.get(0){
        path
    }
    else{
        "./..."
    };
    if let Err(e)=execute_command(path,html_flag,open_flag){
        eprintln!("Error: {}",e);
        exit(1);
    };
}

fn execute_command(path:&str,html_flag:&str,open_flag:&str)->Result<(),Box< dyn std::error::Error>>{
    let mut cmd=Command::new("sh");
    cmd.args(["-s","-",html_flag,path,open_flag]);
    let mut child=cmd.stdin(Stdio::piped()).spawn()?;
    if let Some(stdin)=child.stdin.as_mut(){
        stdin.write_all(GOTEST_SCRIPT.as_bytes())?;
    }
    let status=child.wait()?;
    if !status.success(){
        exit(0);
    };
    Ok(())
}
