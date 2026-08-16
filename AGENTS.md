# Project Working Rules

## Development Philosophy

- This is a learning-by-building Rust project.
- Do not build large amounts of functionality ahead of the requested step.
- Implement features incrementally.
- Prefer a small working implementation over premature architecture.
- Do not over-engineer.
- Do not introduce abstractions merely for hypothetical future requirements.
- Do not redesign the project unless explicitly requested.
- Keep each development step small enough to understand and review.

## Agent Role

- The agent is an engineering assistant, not the primary developer.
- Do not take over the implementation without an explicit request.
- Normal and educational implementation should remain understandable to a Rust beginner.
- Agent-heavy implementation is appropriate mainly for genuinely difficult areas such as:
  - concurrency
  - async architecture
  - complicated ownership/lifetime problems
  - large refactors
  - performance-sensitive implementation
  - difficult debugging
- Never silently replace understandable code with substantially more complex code.

## Rust Learning

- Assume little prior Rust knowledge.
- When a change introduces an important Rust concept, briefly explain it after the implementation.
- Especially explain unfamiliar use of:
  - ownership
  - borrowing
  - references
  - lifetimes
  - Result / Option
  - traits
  - generics
  - iterators
  - closures
  - smart pointers
  - Rc / Arc
  - RefCell / Mutex / RwLock
  - async / await
  - Send / Sync
- Explanations should focus on the code actually being used, not generic Rust tutorials.

## Code Style

- Prefer clear and explicit code.
- Avoid unnecessary cleverness.
- Only wrap source lines when they become longer than approximately 120 characters.
- Do not split short expressions across multiple lines purely for visual formatting.
- Use rustfmt as the formatting authority when applicable.
- Follow idiomatic Rust unless doing so would make a beginner-level implementation unnecessarily obscure.

## Scope Control

- Before editing, inspect the relevant files.
- Modify only files necessary for the current task.
- Do not perform unrelated cleanup.
- Do not perform speculative refactoring.
- Do not add dependencies unless they are needed for the current task.
- Explain why a new dependency is necessary before adding it.
- Do not implement future roadmap items early.

## Validation

After implementation:
- run cargo fmt --check when applicable
- run cargo check
- run relevant tests
- run cargo clippy when appropriate

Do not claim success unless the relevant command actually succeeds.

## Git

- GitHub remotes must use SSH only.
- Never change the remote to HTTPS.
- Do not commit or push unless explicitly requested.
- Before a commit, summarize changed files and validation results.
- Keep commits focused on one logical change.

## Communication

For each implementation task, report:

1. What changed
2. Why it changed
3. Important Rust concepts introduced
4. Validation performed
5. Remaining problems or next logical step

Keep explanations focused on the current implementation.
