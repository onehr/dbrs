use nix::unistd::Pid;
extern crate linenoise;

pub struct Debugger {
    pub m_prog_name: String,
    pub m_pid: Pid,
}

fn callback(input: &str) -> Vec<String> {
    let mut ret: Vec<&str>;
    if input.starts_with("c") {
        ret = vec!["continue"];
    } else if input.starts_with("q") {
        ret = vec!["quit"];
    } else if input.starts_with("e") {
        ret = vec!["exit"];
    } else {
        ret = vec!["continue", "quit", "exit"];
    }
    return ret.iter().map(|s| s.to_string()).collect();
}

impl Debugger {
    fn is_prefix(&self, s: &str, of: &str) -> bool {
        if s.len() > of.len() {
            return false;
        }

        return of.starts_with(s);
    }

    fn continue_exec(&self) -> Result<(), String> {
        // TODO: now can only continue once
        match nix::sys::ptrace::cont(self.m_pid, None) {
            Ok(_ret) => {
            }
            Err(msg) => {
                return dbg!(Err(msg.to_string()));
            }
        }

        match nix::sys::wait::waitpid(self.m_pid, Some(nix::sys::wait::WaitPidFlag::empty())) {
            _ => {}
        }
        return Ok(());
    }

    fn handle_cmd(&self, line: &String) -> Result<bool, String> {
        let args: Vec<&str> = line.split_whitespace().collect();

        if args.is_empty() {
            return Ok(true);
        }

        let cmd = args[0];

        if self.is_prefix(cmd, "continue") {
            self.continue_exec()?;
        } else if self.is_prefix(cmd, "quit") || self.is_prefix(cmd, "exit") {
            return Ok(false);
        } else {
            return Err(format!("Unknown command"));
        }
        return Ok(true);
    }

    pub fn run(&self) {
        match nix::sys::wait::waitpid(self.m_pid, Some(nix::sys::wait::WaitPidFlag::empty())) {
            _ => {}
        }
        linenoise::set_callback(callback);
        loop {
            let line = linenoise::input("(dbrs)> ");
            match line {
                None => break,
                Some(input) => {
                    match self.handle_cmd(&input) {
                        Ok(t) => {
                            if t == false {
                                break;
                            }
                        }
                        Err(msg) => {
                            println!("{}", msg);
                            continue;
                        }
                    }
                    linenoise::history_add(&input);
                }
            }
        }
    }
}
