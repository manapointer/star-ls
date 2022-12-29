use crate::{Diagnostic, SyntaxElement, SyntaxNode, WalkEvent};

pub fn render(syntax: SyntaxNode, diagnostics: Vec<Diagnostic>) -> String {
    let mut buf = String::new();
    let mut indent = 0;

    for event in syntax.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(node) => {
                let text = match &node {
                    SyntaxElement::Token(it) => format!(" {:?}", it.text()),
                    _ => "".to_string(),
                };
                buf.push_str(&format!(
                    "{:indent$}{:?}@{:?}..{:?}{}\n",
                    " ",
                    node.kind(),
                    node.text_range().start(),
                    node.text_range().end(),
                    text,
                    indent = indent
                ));
                indent += 2;
            }
            WalkEvent::Leave(_) => indent -= 2,
        }
    }

    for diagnostic in diagnostics {
        buf.push_str(&format!("{}:{}\n", diagnostic.pos, diagnostic.message));
    }

    buf
}
