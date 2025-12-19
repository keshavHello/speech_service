use std::process::Command;

fn main() {
    let output = Command::new("python")
    .arg("speech.py")
    .output()
    .expect("Failed to run Python speech file");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty(){
        eprintln!("Python error: {}", stderr);
    }
    if output.status.success(){
        println!("Python script finished successfully!");
    } else{
        println!("Python script failed");
    }
}
