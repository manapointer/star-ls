use std::mem;

#[derive(Default, Debug)]
struct CommentBlock {
    lines: Vec<String>,
}

fn extract_comment_blocks(text: &str) -> Vec<CommentBlock> {
    let mut blocks = Vec::new();

    let prefix = "// ";
    let lines = text.lines().map(str::trim_start);

    let mut curr_block = <CommentBlock as Default>::default();
    for line in lines {
        let is_comment = line.starts_with(prefix);
        if is_comment {
            curr_block.lines.push(line[prefix.len()..].to_string());
        } else if !curr_block.lines.is_empty() {
            blocks.push(mem::take(&mut curr_block));
        }
    }
    if !curr_block.lines.is_empty() {
        blocks.push(curr_block);
    }
    blocks
}

pub fn run() -> Result<(), anyhow::Error> {
    let text = "
    
// block1
// block1

// block2
// block2";

    println!("{:?}", extract_comment_blocks(text));

    Ok(())
}
