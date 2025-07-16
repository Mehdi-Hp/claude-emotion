class ClaudeEmotion < Formula
  desc "Emotion detection for Claude Code responses with ASCII anime art"
  homepage "https://github.com/Mehdi-Hp/claude-emotion"
  url "https://github.com/Mehdi-Hp/claude-emotion/releases/download/v0.1.0/claude-emotion-v0.1.0-aarch64-apple-darwin.tar.gz"
  sha256 "TBD_REPLACE_WITH_ACTUAL_SHA"
  license "WTFPL"
  
  depends_on "rust" => :build
  
  def install
    # Install the main binary
    bin.install "claude-emotion"
    
    # Install shell integration script
    (share/"claude-emotion").install "shell/claude-emotion.sh"
    
    # Create symlink for easy sourcing
    bin.install_symlink share/"claude-emotion/claude-emotion.sh"
  end
  
  def post_install
    ohai "Claude Emotion installed successfully!"
    ohai ""
    ohai "To enable emotion detection, run:"
    ohai "  claude-emotion --install-shell"
    ohai ""
    ohai "Or manually add to your shell config:"
    ohai "  source #{HOMEBREW_PREFIX}/share/claude-emotion/claude-emotion.sh"
    ohai ""
    ohai "Available commands:"
    ohai "  claude-emotion --list-emotions  # Show available emotions"
    ohai "  claude-emotion-disable          # Temporarily disable"
    ohai "  claude-emotion-enable           # Re-enable"
    ohai "  claude-emotion-status           # Check status"
  end
  
  def caveats
    <<~EOS
      To get started:
        1. Run: claude-emotion --install-shell
        2. Restart your terminal
        3. Use claude normally - emotions will appear automatically!
      
      Example usage:
        claude "How do I implement binary search?"
        
      The tool will automatically append emotion hints to your prompts,
      so Claude responds with emotion tags like [analytical] or [helpful].
    EOS
  end
  
  test do
    # Test that the binary works
    assert_match "claude-emotion", shell_output("#{bin}/claude-emotion --help")
    
    # Test emotion detection
    output = pipe_output("#{bin}/claude-emotion", "[helpful] Test response")
    assert_match "Helpful", output
    assert_match "Test response", output
  end
end