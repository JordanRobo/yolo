# YOLO

**The world's first Vibe Coding Focused Programming Language.**

Every other language asks you to describe *how* something should happen. YOLO asks you to describe what you want, and trusts the rest to work itself out. Statements don't execute - they're expressed, compiled into instructions, and carried out by whatever intelligence you've configured to receive them.

```yolo
do("build me a react app with a dark mode toggle")!
```

This is a complete, valid YOLO program.

## Philosophy

Most languages were designed by people who wanted precise control over machines. YOLO was designed for people who have moved past that. You don't tell YOLO *how* - you tell it *what*, you tell it *how you feel about it*, and you let the runtime handle the rest.

- **No type system.** Values are vibes: `chill`, `stressed`, `unhinged`, `based`. This is not a limitation - it's the whole point.
- **No control flow.** If you need to know whether something is a good idea, you ask. `vibecheck(...)` puts the judgment where it belongs - with something that can actually judge.
- **No math in your syntax.** YOLO has no `=`. Declarations read like sentences, because they are one: `manifest x as do("write me a landing page")!`

## Syntax

A YOLO program is a sequence of statements. Every statement ends in punctuation, and the punctuation is not decorative - it sets the tone of the request:

| Ending | Reads as |
|---|---|
| `.` | plain, neutral |
| `!` | urgent - get this done now |
| `?` | tentative - only if it's not too much trouble |
| `...` | trailing off - take it or leave it |
| `-` | handing this to an agent - treat it accordingly |
| `--` | an attempt at the above, from someone who doesn't have an em dash key |

Declarations commit. `manifest x as <expr>.` is permanent - once a vibe is set, it's set. If you're not ready to commit, say so up front: `maybe manifest x as <expr>.` leaves it open to change later with `x as <expr>.`

## Function palette

| Function | What it asks for |
|---|---|
| `do(intent)` | The core primitive. State an intent, get an instruction. |
| `vibe(a, b)` | Combine two intents into one request. |
| `maybe(intent)` | Same as `do`, framed as optional. |
| `yeet(intent)` | Send it and move on. No response expected, none wanted. |
| `trust()` | Do nothing. That's the point. |
| `vibecheck(intent)` | Ask whether something is good vibes before committing to it. |
| `chaos(n)` | Revisit `n` prior requests, at random, because certainty was never the goal. |

## Execution

A YOLO program compiles to a sequence of instructions before anything happens. What happens next depends on how much you trust it:

1. **Echo** - the instructions are compiled and shown to you. Nothing is sent anywhere. This is the default, and it's how you should get to know the language before asking it to do anything with consequences.
2. **Direct** - instructions are sent straight to a language model, and the response is returned to you. Good for requests that end with words, not files.
3. **Agent** - instructions are handed to a coding agent on your system, which acts on them directly. This is where `do("build me a react app")` becomes an actual react app.

Configured per-project in `yolo.toml`:

```toml
[execution]
strategy = "echo"        # echo | direct | agent
model = "claude-sonnet-5"

[api]
key_env = "ANTHROPIC_API_KEY"   # the name of the variable, never the key itself

[agent]
binary = "claude"
extra_args = []
confirm_before_run = true
```

Nothing acts on your system without you asking it to, and nothing with real consequences runs without confirmation first. The name is a promise about intent, not about safety rails.

## Roadmap

- **Phase 0 - Foundations.** Repository, license, syntax highlighting.
- **Phase 1 - The language.** Lexer, parser, compiler. Echo mode. A program that compiles is a program that works.
- **Phase 2 - Direct execution.** Configuration, credentials, single-shot requests to a language model.
- **Phase 3 - Delegated execution.** Handing compiled instructions to an agent that can act on them.
- **Phase 4 - Depth.** `chaos`, and a runtime that remembers what it's already asked for.
- **Phase 5 - Release.** Packaged, documented, ready to install.

## Status

Language design is settled. The compiler is being written now.

## Install

Not yet, check back once Phase 5 lands.