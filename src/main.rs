use std::io::{self, BufRead, BufReader, Write};
use regex::Regex;
use once_cell::sync::Lazy;

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

fn main() {
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