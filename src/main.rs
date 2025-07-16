use std::io::{self, BufRead, BufReader, Write};
use std::fs;
use std::path::Path;
use clap::{Arg, Command};
use claude_emotion::{parse_emotion_tag, format_emotion_box};

fn install_shell_integration() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let shell_configs = vec![
        (".zshrc", "zsh"),
        (".bashrc", "bash"),
        (".config/fish/config.fish", "fish"),
    ];
    
    let _script_content = include_str!("../shell/claude-emotion.sh");
    let source_line = "source /opt/homebrew/bin/claude-emotion.sh";
    let marker = "# Claude Emotion Integration";
    
    let mut installed_count = 0;
    
    for (config_file, shell_name) in shell_configs {
        let config_path = Path::new(&home_dir).join(config_file);
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            
            if !content.contains(marker) {
                let new_content = format!("\n{}\n{}\n", marker, source_line);
                fs::write(&config_path, content + &new_content)?;
                println!("✓ Added to {} ({})", config_file, shell_name);
                installed_count += 1;
            } else {
                println!("✓ Already configured in {} ({})", config_file, shell_name);
            }
        }
    }
    
    if installed_count == 0 {
        println!("No shell configuration files found to modify.");
        println!("Please manually add this line to your shell config:");
        println!("  {}", source_line);
    } else {
        println!("\nInstallation complete! Please restart your terminal or run:");
        println!("  source ~/.zshrc");
        println!("  # or");
        println!("  source ~/.bashrc");
    }
    
    Ok(())
}

fn list_emotions() {
    println!("Available emotions:");
    println!("  analytical  - Deep thinking, examining");
    println!("  helpful     - Eager to assist");
    println!("  curious     - Wondering, exploring");
    println!("  uncertain   - Not sure, hesitant");
    println!("  apologetic  - Sorry, corrections");
    println!("  excited     - Enthusiastic, happy");
}

fn run_stream_processor() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = io::stdout();
    
    let mut buffer = String::new();
    let mut emotion_displayed = false;
    
    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                // Check for emotion tag on the first line
                if !emotion_displayed {
                    if let Some((emotion, cleaned_line)) = parse_emotion_tag(&buffer) {
                        // Print the emotion box
                        print!("{}", format_emotion_box(&emotion));
                        emotion_displayed = true;
                        
                        // Output the cleaned line (without the tag)
                        stdout.write_all(cleaned_line.as_bytes()).unwrap();
                        stdout.flush().unwrap();
                        continue;
                    }
                }
                
                // Pass through the line as-is
                stdout.write_all(buffer.as_bytes()).unwrap();
                stdout.flush().unwrap();
            }
            Err(_) => break,
        }
    }
}

fn main() {
    let matches = Command::new("claude-emotion")
        .version("0.1.0")
        .author("Mehdi Hoseini")
        .about("Emotion detection for Claude Code responses with ASCII anime art")
        .arg(
            Arg::new("install-shell")
                .long("install-shell")
                .help("Install shell integration automatically")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("list-emotions")
                .long("list-emotions")
                .help("List available emotions")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();
    
    if matches.get_flag("install-shell") {
        if let Err(e) = install_shell_integration() {
            eprintln!("Error installing shell integration: {}", e);
            std::process::exit(1);
        }
        return;
    }
    
    if matches.get_flag("list-emotions") {
        list_emotions();
        return;
    }
    
    // Default behavior: run as stream processor
    run_stream_processor();
}