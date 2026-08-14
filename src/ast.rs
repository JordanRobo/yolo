#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Plain,        // .
    Urgent,       // !
    Tentative,    // ?
    TrailingOff,  // ...
    AgentHandoff, // - (a real em dash: greet the agent, oversell the request)
    Impersonator, // -- (ascii fake: caught and mocked)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// `manifest x as <expr>;` (mutable = false)
    /// `maybe manifest x as <expr>;` (mutable = true)
    Manifest { name: String, value: Expr, mutable: bool, tone: Tone, line: usize },
    /// `x as <expr>;` - only valid if `x` was manifested as mutable.
    /// Enforced at the compiler stage, not the parser.
    Reassign { name: String, value: Expr, tone: Tone, line: usize },
    /// A bare expression statement, e.g. `do("...");`
    Expr { expr: Expr, tone: Tone, line: usize },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A function call, e.g. `do("build an app")`, `vibe(a, b)`
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    StringLit(String),
    VibeLit(Vibe),
    /// Reference to a previously manifested name
    Ident(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vibe {
    Chill,
    Stressed,
    Unhinged,
    Based,
}

/// A full parsed program - top-level statements in source order.
pub type Program = Vec<Stmt>;