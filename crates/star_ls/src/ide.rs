pub(crate) struct Lines {
    positions: Vec<usize>,
}

impl Lines {
    pub(crate) fn from_str(s: &str) -> Lines {
        let mut positions = Vec::new();
        let mut cursor = 0;

        for ch in s.chars() {
            if ch == '\n' {
                positions.push(cursor);
            }
            cursor += ch.len_utf8();
        }

        Lines { positions }
    }

    pub(crate) fn line_num_and_col(&self, pos: usize) -> lsp_types::Position {
        let index = self.positions.partition_point(|line_pos| *line_pos < pos);

        // First line. Simply return position.
        let (line, character) = if index == 0 {
            (0, pos as u32)
        } else {
            (index as u32, (pos - self.positions[index - 1] - 1) as u32)
        };
        lsp_types::Position { line, character }
    }
}
