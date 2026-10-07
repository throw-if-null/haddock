#!/usr/bin/env bats
# The PostToolUse hook in hooks/check-markdown and its configuration file. Each test
# writes a hook event as JSON to the hook's stdin, the way Claude Code does.

setup() {
	load test_helper
	HOOK="$BATS_TEST_DIRNAME/../hooks/check-markdown"
	cd "$BATS_TEST_TMPDIR" || return 1
	mkdir .git
	cp "$BATS_TEST_DIRNAME/fail/chain.md" chain.md
	cp "$BATS_TEST_DIRNAME/pass/words.md" words.md
}

# event TOOL FILE prints a Claude Code hook event for an Edit or Write of FILE.
event() {
	jq -n --arg tool "$1" --arg file "$2" --arg cwd "$PWD" \
		'{hook_event_name: "PostToolUse", tool_name: $tool, cwd: $cwd, tool_input: {file_path: $file}}'
}

@test "hook: an Edit of a Markdown file with candidates prints them for the agent and the user" {
	run -0 "$HOOK" < <(event Edit chain.md)
	jq -e '.hookSpecificOutput.hookEventName == "PostToolUse"' <<<"$output"
	context="$(jq -r '.hookSpecificOutput.additionalContext' <<<"$output")"
	[[ "$context" == "chain.md:3: [chain] —"* ]]
	[[ "$context" == *"Candidates reported."* ]]
	[ "$(jq -r '.systemMessage' <<<"$output")" = "$context" ]
}

@test "hook: a Write of a Markdown file without candidates prints nothing" {
	run -0 "$HOOK" < <(event Write words.md)
	[ -z "$output" ]
}

@test "hook: an absolute path is reported as given" {
	run -0 "$HOOK" < <(event Edit "$PWD/chain.md")
	[[ "$(jq -r '.systemMessage' <<<"$output")" == "$PWD/chain.md:3:"* ]]
}

@test "hook: a file that is not Markdown is not checked" {
	cp chain.md notes.txt
	run -0 "$HOOK" < <(event Edit notes.txt)
	[ -z "$output" ]
}

@test "hook: a Markdown file that does not exist is not checked" {
	run -0 "$HOOK" < <(event Edit missing.md)
	[ -z "$output" ]
}

@test "hook: the .doc-style file of the project applies" {
	printf 'chain.md chain\n' >.doc-style
	run -0 "$HOOK" < <(event Edit chain.md)
	[ -z "$output" ]
}

@test "hook: a checker error is reported to the agent and the user" {
	printf 'chain.md tnoe\n' >.doc-style
	run -0 "$HOOK" < <(event Edit chain.md)
	message="$(jq -r '.systemMessage' <<<"$output")"
	[[ "$message" == "doc-style checker failed (exit 2):"* ]]
	[[ "$message" == *"unknown rule: tnoe"* ]]
}

@test "hook: input that is not JSON exits 0 and reports on stderr" {
	run -0 --separate-stderr "$HOOK" <<<"not json"
	[ -z "$output" ]
	[[ "$stderr" == *"stdin is not JSON"* ]]
}

@test "hook: DOC_STYLE_CHECK selects the checker" {
	printf '#!/usr/bin/env bash\necho "stub finding"; exit 1\n' >stub-check
	chmod +x stub-check
	DOC_STYLE_CHECK="$PWD/stub-check" run -0 "$HOOK" < <(event Edit words.md)
	[ "$(jq -r '.systemMessage' <<<"$output")" = "stub finding" ]
}

@test "config: the hook file is valid JSON, matches PostToolUse, and runs check-markdown" {
	jq -e '.hooks.PostToolUse[0].hooks[0] | .type == "command" and (.command | test("check-markdown"))' \
		"$BATS_TEST_DIRNAME/../hooks/claude-code.json"
}

@test "config: the Claude Code matcher covers Edit and Write" {
	matcher="$(jq -r '.hooks.PostToolUse[0].matcher' "$BATS_TEST_DIRNAME/../hooks/claude-code.json")"
	[[ Edit =~ ^($matcher)$ ]]
	[[ Write =~ ^($matcher)$ ]]
}
