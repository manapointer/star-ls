pub struct Lines {
    positions: Vec<usize>,
}

impl Lines {
    pub fn new(s: &str) -> Lines {
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

    pub fn line_num_and_col(&self, pos: usize) -> (u32, u32) {
        let index = self.positions.partition_point(|line_pos| *line_pos < pos);

        // First line. Simply return position.
        if index == 0 {
            (0, pos as u32)
        } else {
            (index as u32, (pos - self.positions[index - 1] - 1) as u32)
        }
    }
}
