use gorani_syntax::SyntaxNode;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Diagnostics {
    diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Diagnostic {
    message: String,
    node: SyntaxNode,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub fn report(&mut self, message: &str, node: &SyntaxNode) {
        self.diagnostics.push(Diagnostic {
            message: message.to_owned(),
            node: node.clone(),
        });
    }

    pub fn print(&self) {
        for diagnostic in &self.diagnostics {
            println!(
                "Error: {} at {:?}",
                diagnostic.message,
                diagnostic.node.text_range()
            );
            println!("{}", diagnostic.node.text());
            println!();
        }
    }
}
