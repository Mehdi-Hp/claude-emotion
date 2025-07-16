# Claude Emotion 🎭

A lightweight Rust CLI tool that detects emotional tone in Claude Code responses and displays ASCII anime art in beautiful terminal boxes.

## Features

- **Transparent Integration**: Works seamlessly with existing `claude` command
- **Real-time Detection**: Processes streaming output without delay  
- **ASCII Art Display**: Shows emotion-themed anime characters in Unicode boxes
- **Easy Control**: Enable/disable with simple commands
- **Zero Dependencies**: Single static binary, no runtime requirements
- **Cross-platform**: Works on macOS, Linux, and Windows

## Preview

```bash
$ claude "How do I implement binary search?"

╭─────────────────────╮
│     Analytical      │
├─────────────────────┤
│     ∧_∧            │
│   ( ･ω･)つ━☆       │
│   (つ　 /           │
│    |  (⌒)          │
│    し⌒             │
╰─────────────────────╯

I'll help you implement a binary search algorithm...
```

## Installation

### Homebrew (Recommended)

```bash
# Add tap
brew tap mehdi-hoseini/claude-emotion

# Install
brew install claude-emotion

# Setup shell integration
claude-emotion --install-shell

# Restart terminal and enjoy!
```

### Manual Installation

1. Download the latest binary from [releases](https://github.com/mehdi-hoseini/claude-emotion/releases)
2. Place in your PATH
3. Run `claude-emotion --install-shell`

## How It Works

1. **Prompt Injection**: The tool automatically appends emotion hints to your prompts
2. **Claude Response**: Claude naturally includes emotion tags like `[analytical]` or `[helpful]`
3. **Tag Parsing**: The tool detects these tags and displays appropriate ASCII art
4. **Clean Output**: Tags are removed from the final output

## Available Emotions

- **analytical** - Deep thinking, examining → `( ･ω･)つ━☆`
- **helpful** - Eager to assist → `( ＾ω＾)b`
- **curious** - Wondering, exploring → `( ･o･)?`
- **uncertain** - Not sure, hesitant → `(・_・;)`
- **apologetic** - Sorry, corrections → `m(_ _)m`
- **excited** - Enthusiastic, happy → `(★ω★)`

## Usage

Once installed, just use `claude` normally:

```bash
claude "Explain recursion"
claude "Fix this bug in my code"
claude "What's the best way to learn Rust?"
```

## Control Commands

```bash
# Check status
claude-emotion-status

# Temporarily disable
claude-emotion-disable

# Re-enable
claude-emotion-enable

# List available emotions
claude-emotion --list-emotions
```

## Configuration

The tool works out of the box with sensible defaults. Advanced users can:

- Disable globally: `export CLAUDE_EMOTION_DISABLED=true`
- View help: `claude-emotion --help`

## Technical Details

- **Language**: Rust 2024 edition
- **Dependencies**: regex, clap, once_cell
- **Binary Size**: ~1.3MB
- **Performance**: < 10ms added latency
- **Compatibility**: Works with all Claude Code features

## Development

```bash
# Build from source
git clone https://github.com/mehdi-hoseini/claude-emotion
cd claude-emotion
cargo build --release

# Test
echo "[helpful] Test response" | ./target/release/claude-emotion
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details.

## Author

Created by [Mehdi Hoseini](https://github.com/mehdi-hoseini) with assistance from Claude.

---

*Make your terminal interactions more expressive! ✨*