use std::io::{self, BufRead, BufReader, Write};
use std::fs;
use std::path::Path;
use regex::Regex;
use once_cell::sync::Lazy;
use clap::{Arg, Command};

#[derive(Debug, Clone, Copy)]
enum Emotion {
    Analytical,
    Helpful,
    Curious,
    Uncertain,
    Apologetic,
    Excited,
}

impl Emotion {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "analytical" => Some(Emotion::Analytical),
            "helpful" => Some(Emotion::Helpful),
            "curious" => Some(Emotion::Curious),
            "uncertain" => Some(Emotion::Uncertain),
            "apologetic" => Some(Emotion::Apologetic),
            "excited" => Some(Emotion::Excited),
            _ => None,
        }
    }

    fn title(&self) -> &'static str {
        match self {
            Emotion::Analytical => "Analytical",
            Emotion::Helpful => "Helpful",
            Emotion::Curious => "Curious",
            Emotion::Uncertain => "Uncertain",
            Emotion::Apologetic => "Apologetic",
            Emotion::Excited => "Excited",
        }
    }

    fn art(&self) -> &'static str {
        match self {
            Emotion::Analytical => r#"
     ∧_∧
   ( ･ω･)つ━☆
   (つ　 /
    |  (⌒)
    し⌒"#,
            Emotion::Helpful => r#"
     ∧_∧
   ( ＾ω＾)b
   (つ　 /
    |  (⌒)
    し⌒"#,
            Emotion::Curious => r#"
     ∧_∧
   ( ･o･)?
   (つ　 /
    |  (⌒)
    し⌒"#,
            Emotion::Uncertain => r#"
    (・_・;)
      💧"#,
            Emotion::Apologetic => r#"
     ∧_∧
   m(_ _)m"#,
            Emotion::Excited => r#"
     ∧_∧
   (★ω★)
   (つ　 /
    |  (⌒)
    し⌒"#,
        }
    }
}

static EMOTION_TAG_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\[(\w+)\]\s*").unwrap()
});

fn parse_emotion_tag(line: &str) -> Option<(Emotion, String)> {
    if let Some(captures) = EMOTION_TAG_REGEX.captures(line) {
        if let Some(emotion_str) = captures.get(1) {
            if let Some(emotion) = Emotion::from_str(emotion_str.as_str()) {
                let cleaned_line = EMOTION_TAG_REGEX.replace(line, "").to_string();
                return Some((emotion, cleaned_line));
            }
        }
    }
    None
}

fn display_emotion_box(emotion: &Emotion) {
    let title = emotion.title();
    let art = emotion.art();
    
    println!("╭─────────────────────╮");
    println!("│{:^21}│", title);
    println!("├─────────────────────┤");
    
    for line in art.trim().lines() {
        println!("│{:<21}│", line);
    }
    
    println!("╰─────────────────────╯");
    println!();
}

fn install_shell_integration() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let shell_configs = vec![
        (".zshrc", "zsh"),
        (".bashrc", "bash"),
        (".config/fish/config.fish", "fish"),
    ];
    
    let script_content = include_str!("../shell/claude-emotion.sh");
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
                        display_emotion_box(&emotion);
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