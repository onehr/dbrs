mod debugger;
use std::{env, ffi};

extern crate nix;
use nix::unistd::{fork, ForkResult};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("no input file"));
    }
    let obj_name = &args[1];

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            // the parent process
            // execute debugger
            let dgb = debugger::Debugger {
                m_prog_name: obj_name.clone(),
                m_pid: child,
            };
            dgb.run();
        }
        Ok(ForkResult::Child) => {
            // the child process
            // execute the debugged program
            match nix::sys::ptrace::traceme() {
                Ok(()) => {}
                Err(msg) => {
                    return Err(msg.to_string());
                }
            }
            // XXX: now only support file path like ./a.out or full path
            match nix::unistd::execvp(
                &ffi::CString::new(obj_name.clone()).unwrap(),
                &[ffi::CString::new(obj_name.clone()).unwrap()],
            ) {
                Ok(_s) => {

                }
                Err(msg) => {
                    return Err(msg.to_string());
                }
            }
        }
        Err(_) => println!("Fork failed"),
    }

    return Ok(());
}
