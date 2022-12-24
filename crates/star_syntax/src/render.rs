use crate::{Diagnostic, SyntaxElement, SyntaxNode, WalkEvent};

pub fn render(syntax: SyntaxNode, diagnostics: Vec<Diagnostic>) -> String {
    let mut buf = String::new();
    let mut indent = 0;
    let mut start = 0;
    let mut pos = 0;

    for event in syntax.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(node) => {
                let text = match &node {
                    SyntaxElement::Node(it) => it.text().to_string(),
                    SyntaxElement::Token(it) => {
                        start = pos;
                        pos += it.text().len();
                        it.text().to_string()
                    }
                };
                buf.push_str(&format!(
                    "{:indent$}{:?}@{}..{} {:?}\n",
                    " ",
                    node.kind(),
                    start,
                    pos,
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
