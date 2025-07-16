// Remove unused imports - only need regex and once_cell for lib
use regex::Regex;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Emotion {
    Analytical,
    Helpful,
    Curious,
    Uncertain,
    Apologetic,
    Excited,
}

impl Emotion {
    pub fn from_str(s: &str) -> Option<Self> {
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

    pub fn title(&self) -> &'static str {
        match self {
            Emotion::Analytical => "Analytical",
            Emotion::Helpful => "Helpful",
            Emotion::Curious => "Curious",
            Emotion::Uncertain => "Uncertain",
            Emotion::Apologetic => "Apologetic",
            Emotion::Excited => "Excited",
        }
    }

    pub fn art(&self) -> &'static str {
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

pub fn parse_emotion_tag(line: &str) -> Option<(Emotion, String)> {
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

pub fn format_emotion_box(emotion: &Emotion) -> String {
    let title = emotion.title();
    let art = emotion.art();
    
    let mut result = String::new();
    result.push_str("╭─────────────────────╮\n");
    result.push_str(&format!("│{:^21}│\n", title));
    result.push_str("├─────────────────────┤\n");
    
    for line in art.trim().lines() {
        result.push_str(&format!("│{:<21}│\n", line));
    }
    
    result.push_str("╰─────────────────────╯\n");
    result.push('\n');
    
    result
}

pub fn process_line(line: &str, emotion_displayed: &mut bool) -> String {
    if !*emotion_displayed {
        if let Some((emotion, cleaned_line)) = parse_emotion_tag(line) {
            *emotion_displayed = true;
            return format!("{}{}", format_emotion_box(&emotion), cleaned_line);
        }
    }
    line.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_from_str() {
        assert_eq!(Emotion::from_str("analytical"), Some(Emotion::Analytical));
        assert_eq!(Emotion::from_str("HELPFUL"), Some(Emotion::Helpful));
        assert_eq!(Emotion::from_str("Curious"), Some(Emotion::Curious));
        assert_eq!(Emotion::from_str("invalid"), None);
        assert_eq!(Emotion::from_str(""), None);
    }

    #[test]
    fn test_emotion_title() {
        assert_eq!(Emotion::Analytical.title(), "Analytical");
        assert_eq!(Emotion::Helpful.title(), "Helpful");
        assert_eq!(Emotion::Curious.title(), "Curious");
    }

    #[test]
    fn test_emotion_art() {
        assert!(Emotion::Analytical.art().contains("( ･ω･)つ━☆"));
        assert!(Emotion::Helpful.art().contains("( ＾ω＾)b"));
        assert!(Emotion::Uncertain.art().contains("(・_・;)"));
    }

    #[test]
    fn test_parse_emotion_tag_valid() {
        let result = parse_emotion_tag("[analytical] This is a test");
        assert!(result.is_some());
        let (emotion, cleaned) = result.unwrap();
        assert_eq!(emotion, Emotion::Analytical);
        assert_eq!(cleaned, "This is a test");
    }

    #[test]
    fn test_parse_emotion_tag_with_extra_spaces() {
        let result = parse_emotion_tag("[helpful]   Extra spaces");
        assert!(result.is_some());
        let (emotion, cleaned) = result.unwrap();
        assert_eq!(emotion, Emotion::Helpful);
        assert_eq!(cleaned, "Extra spaces");
    }

    #[test]
    fn test_parse_emotion_tag_invalid() {
        assert!(parse_emotion_tag("[invalid] Test").is_none());
        assert!(parse_emotion_tag("No tag here").is_none());
        assert!(parse_emotion_tag("[analytical extra] Malformed").is_none());
        assert!(parse_emotion_tag("").is_none());
    }

    #[test]
    fn test_parse_emotion_tag_case_insensitive() {
        let result = parse_emotion_tag("[EXCITED] Test");
        assert!(result.is_some());
        let (emotion, _) = result.unwrap();
        assert_eq!(emotion, Emotion::Excited);
    }

    #[test]
    fn test_format_emotion_box() {
        let box_output = format_emotion_box(&Emotion::Analytical);
        assert!(box_output.contains("╭─────────────────────╮"));
        assert!(box_output.contains("│     Analytical      │"));
        assert!(box_output.contains("├─────────────────────┤"));
        assert!(box_output.contains("╰─────────────────────╯"));
        assert!(box_output.contains("( ･ω･)つ━☆"));
    }

    #[test]
    fn test_process_line_with_emotion() {
        let mut emotion_displayed = false;
        let result = process_line("[curious] This is interesting", &mut emotion_displayed);
        
        assert!(emotion_displayed);
        assert!(result.contains("Curious"));
        assert!(result.contains("This is interesting"));
        assert!(result.contains("╭─────────────────────╮"));
    }

    #[test]
    fn test_process_line_without_emotion() {
        let mut emotion_displayed = false;
        let result = process_line("Regular line", &mut emotion_displayed);
        
        assert!(!emotion_displayed);
        assert_eq!(result, "Regular line");
    }

    #[test]
    fn test_process_line_emotion_already_displayed() {
        let mut emotion_displayed = true;
        let result = process_line("[analytical] Should not show box", &mut emotion_displayed);
        
        assert!(emotion_displayed);
        assert_eq!(result, "[analytical] Should not show box");
    }

    #[test]
    fn test_utf8_handling() {
        let result = parse_emotion_tag("[excited] 这是中文测试! 🎉 émojis");
        assert!(result.is_some());
        let (emotion, cleaned) = result.unwrap();
        assert_eq!(emotion, Emotion::Excited);
        assert_eq!(cleaned, "这是中文测试! 🎉 émojis");
    }

    #[test]
    fn test_multiline_content() {
        let input = "[analytical] Line 1\nLine 2\nLine 3";
        let mut emotion_displayed = false;
        let result = process_line(input, &mut emotion_displayed);
        
        assert!(emotion_displayed);
        assert!(result.contains("Analytical"));
        assert!(result.contains("Line 1\nLine 2\nLine 3"));
    }

    #[test]
    fn test_empty_input() {
        assert!(parse_emotion_tag("").is_none());
        
        let mut emotion_displayed = false;
        let result = process_line("", &mut emotion_displayed);
        assert!(!emotion_displayed);
        assert_eq!(result, "");
    }

    #[test]
    fn test_very_long_input() {
        let long_content = "a".repeat(10000);
        let input = format!("[helpful] {}", long_content);
        
        let result = parse_emotion_tag(&input);
        assert!(result.is_some());
        let (emotion, cleaned) = result.unwrap();
        assert_eq!(emotion, Emotion::Helpful);
        assert_eq!(cleaned, long_content);
    }

    #[test]
    fn test_all_emotions_parsing() {
        let test_cases = vec![
            ("[analytical] test", Emotion::Analytical),
            ("[helpful] test", Emotion::Helpful),
            ("[curious] test", Emotion::Curious),
            ("[uncertain] test", Emotion::Uncertain),
            ("[apologetic] test", Emotion::Apologetic),
            ("[excited] test", Emotion::Excited),
        ];

        for (input, expected_emotion) in test_cases {
            let result = parse_emotion_tag(input);
            assert!(result.is_some(), "Failed to parse: {}", input);
            let (emotion, _) = result.unwrap();
            assert_eq!(emotion, expected_emotion, "Wrong emotion for: {}", input);
        }
    }
}