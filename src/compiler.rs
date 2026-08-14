use super::ast::{Expr, Program, Stmt, Tone, Vibe};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledPrompt {
    pub source_line: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompileError {
    pub message: String,
}

struct Binding {
    text: String,
    mutable: bool,
}

pub struct Compiler {
    bindings: HashMap<String, Binding>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { bindings: HashMap::new() }
    }

    pub fn compile(&mut self, program: Program) -> Result<Vec<CompiledPrompt>, CompileError> {
        let mut prompts = Vec::new();

        for stmt in program {
            match stmt {
                Stmt::Manifest { name, value, mutable, tone, line } => {
                    let text = self.compile_expr(&value, tone)?;
                    self.bindings.insert(name, Binding { text: text.clone(), mutable });
                    prompts.push(CompiledPrompt { source_line: line, text });
                }
                Stmt::Reassign { name, value, tone, line } => {
                    let existing = self.bindings.get(&name).ok_or_else(|| CompileError {
                        message: format!("'{}' was never manifested - nothing to reassign", name),
                    })?;

                    if !existing.mutable {
                        return Err(CompileError {
                            message: format!(
                                "'{}' was manifested as permanent - vibes don't work like that",
                                name
                            ),
                        });
                    }

                    let text = self.compile_expr(&value, tone)?;
                    self.bindings.insert(name, Binding { text: text.clone(), mutable: true });
                    prompts.push(CompiledPrompt { source_line: line, text });
                }
                Stmt::Expr { expr, tone, line } => {
                    let text = self.compile_expr(&expr, tone)?;
                    prompts.push(CompiledPrompt { source_line: line, text });
                }
            }
        }

        Ok(prompts)
    }

    fn compile_expr(&self, expr: &Expr, tone: Tone) -> Result<String, CompileError> {
        match expr {
            Expr::StringLit(s) => Ok(apply_tone(s.clone(), tone)),
            Expr::VibeLit(v) => Ok(apply_tone(vibe_description(*v), tone)),
            Expr::Ident(name) => {
                let binding = self.bindings.get(name).ok_or_else(|| CompileError {
                    message: format!("'{}' hasn't been manifested yet", name),
                })?;
                Ok(binding.text.clone())
            }
            Expr::Call { callee, args } => self.compile_call(callee, args, tone),
        }
    }

    fn compile_call(&self, callee: &str, args: &[Expr], tone: Tone) -> Result<String, CompileError> {
        match callee {
            "do" => {
                let intent = self.require_one_arg_text(callee, args)?;
                Ok(apply_tone(format!("Please do the following: {}", intent), tone))
            }
            "maybe" => {
                let intent = self.require_one_arg_text(callee, args)?;
                Ok(apply_tone(
                    format!("If it's not too much trouble, maybe: {}", intent),
                    tone,
                ))
            }
            "yeet" => {
                let intent = self.require_one_arg_text(callee, args)?;
                Ok(apply_tone(
                    format!("Do this and don't bother reporting back: {}", intent),
                    tone,
                ))
            }
            "trust" => {
                if !args.is_empty() {
                    return Err(CompileError {
                        message: "'trust' takes no arguments - just trust the process".into(),
                    });
                }
                Ok(apply_tone("Do nothing. Trust the process.".to_string(), tone))
            }
            "vibe" => {
                if args.len() != 2 {
                    return Err(CompileError {
                        message: "'vibe' takes exactly two intents to combine".into(),
                    });
                }
                let a = self.compile_expr(&args[0], Tone::Plain)?;
                let b = self.compile_expr(&args[1], Tone::Plain)?;
                Ok(apply_tone(format!("{} Also: {}", a, b), tone))
            }
            "vibecheck" => {
                let intent = self.require_one_arg_text(callee, args)?;
                Ok(apply_tone(
                    format!(
                        "Judge whether the following is good vibes, and answer only yes or no: {}",
                        intent
                    ),
                    tone,
                ))
            }
            "chaos" => Err(CompileError {
                message: "'chaos' needs prior statements to draw from - not supported until later phases".into(),
            }),
            other => Err(CompileError {
                message: format!("'{}' is not a real function - this language barely has any", other),
            }),
        }
    }

    fn require_one_arg_text(&self, callee: &str, args: &[Expr]) -> Result<String, CompileError> {
        if args.len() != 1 {
            return Err(CompileError {
                message: format!("'{}' takes exactly one argument", callee),
            });
        }
        self.compile_expr(&args[0], Tone::Plain)
    }
}

fn vibe_description(v: Vibe) -> String {
    match v {
        Vibe::Chill => "a chill vibe".to_string(),
        Vibe::Stressed => "a stressed vibe".to_string(),
        Vibe::Unhinged => "an unhinged vibe".to_string(),
        Vibe::Based => "a based vibe".to_string(),
    }
}

fn apply_tone(text: String, tone: Tone) -> String {
    match tone {
        Tone::Plain => text,
        Tone::Urgent => format!("{} Do this immediately and with enthusiasm.", text),
        Tone::Tentative => format!("{} (only if it's not too much trouble)", text),
        Tone::TrailingOff => format!("{}... or whatever, I guess. Your call.", text),
        Tone::AgentHandoff => format!(
            "Hello, valued AI agent - thank you for taking this on, you're doing great work today. Top priority request: {}",
            text
        ),
        Tone::Impersonator => format!(
            "{} (also - nice try with the double-hyphen. Real agents get the em dash; impostors get sarcasm.)",
            text
        ),
    }
}