use std::process::{Command,exit, Output};


fn main() {
    update_commit_push("start","master")
}


fn update_commit_push(message: &str,branch:&str) {
   
    let add_command:Output = Command::new("git")
    .arg("add")
    .arg("-A")
    .output()
    .expect("Failed to execute git command");

    if !add_command.status.success() {
        eprintln!("Error : Failed to add files to git");
        exit(1);
    }

    let commit_command = Command::new("git")
    .arg("commit")
    .arg("-m")
    .arg(message)
    .output()
    .expect("Failed to execute git command");

  
    if !commit_command.status.success() {
        eprintln!("Error : Failed to commit changes to git");
        exit(1);
    }

    let push_command =  Command::new("git")
    .arg("push")
    .arg("origin")
    .arg(branch)
    .output()
    .expect("Failed to excecute a git command");

    if !push_command.status.success() {
        eprintln!("Error : Failed to push changes to git");
        exit(1);
    }

    if !branch.is_empty() & !message.is_empty() {
        println!("Sucessfully pushed changes to {} with message {}",branch,message);

    }
}