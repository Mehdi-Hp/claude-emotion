#!/bin/bash
# Claude Emotion Shell Integration
# This script overrides the claude command to add emotion detection

# Check if claude command exists
if ! command -v claude &> /dev/null; then
    echo "Warning: claude command not found. Please install Claude Code first."
    return 2>/dev/null || exit 1
fi

# Check if claude-emotion binary is in PATH
if ! command -v claude-emotion &> /dev/null; then
    echo "Warning: claude-emotion not found in PATH. Please install claude-emotion first."
    return 2>/dev/null || exit 1
fi

# Override claude command with emotion detection
claude() {
    # Check if emotion detection is disabled
    if [[ "$CLAUDE_EMOTION_DISABLED" == "true" ]]; then
        command claude "$@"
        return
    fi
    
    # Inject emotion hint into the prompt
    local args=("$@")
    local last_idx=$((${#args[@]} - 1))
    
    if [[ $last_idx -ge 0 ]]; then
        # Add emotion hint to the last argument (the prompt)
        local emotion_hint=" [Start with: analytical/helpful/curious/uncertain/apologetic/excited]"
        args[$last_idx]="${args[$last_idx]}${emotion_hint}"
    fi
    
    # Run claude with modified arguments and pipe through emotion detector
    command claude "${args[@]}" | claude-emotion
}

# Convenience functions
claude-emotion-disable() {
    export CLAUDE_EMOTION_DISABLED=true
    echo "Claude emotion detection disabled"
}

claude-emotion-enable() {
    unset CLAUDE_EMOTION_DISABLED
    echo "Claude emotion detection enabled"
}

claude-emotion-status() {
    if [[ "$CLAUDE_EMOTION_DISABLED" == "true" ]]; then
        echo "Claude emotion detection: DISABLED"
    else
        echo "Claude emotion detection: ENABLED"
    fi
}

# Export functions for subshells
export -f claude
export -f claude-emotion-disable
export -f claude-emotion-enable
export -f claude-emotion-status

# Display status on load
echo "Claude emotion detection loaded. Use 'claude-emotion-status' to check status."