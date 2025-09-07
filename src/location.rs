pub struct Location {
    file: String,
    line_start: u64,
    column_start: u64,
    line_end: u64,
    column_end: u64,
}

pub fn build_location(
    file: String,
    line_start: u64,
    column_start: u64,
    line_end: u64,
    column_end: u64,
) -> Location {
    Location {
        file: file,
        line_start: line_start,
        column_start: column_start,
        line_end: line_end,
        column_end: column_end,
    }
}

impl ToString for Location {
    fn to_string(&self) -> String {
        if self.line_start == self.line_end {
            if self.column_start == self.column_end {
                self.file.clone()
                    + ":"
                    + &self.line_start.to_string()
                    + ":"
                    + &self.column_start.to_string()
            } else {
                self.file.clone()
                    + ":"
                    + &self.line_start.to_string()
                    + ":"
                    + &self.column_start.to_string()
                    + "-"
                    + &self.column_end.to_string()
            }
        } else {
            self.file.clone()
                + ":"
                + &self.line_start.to_string()
                + ":"
                + &self.column_start.to_string()
                + "-"
                + &self.line_end.to_string()
                + ":"
                + &self.column_end.to_string()
        }
    }
}

impl Clone for Location {
    fn clone(&self) -> Location {
        Location {
            file: self.file.clone(),
            line_start: self.line_start,
            column_start: self.column_start,
            line_end: self.line_end,
            column_end: self.column_end,
        }
    }
}

impl Location {
    pub fn start_to_end(&mut self) {
        self.line_start = self.line_end;
        self.column_start = self.column_end;
    }

    pub fn step_line(&mut self) {
        self.line_end += 1;
        self.column_end = 0;
    }

    pub fn step_column(&mut self) {
        self.column_end += 1;
    }

    pub fn end_to_next(&mut self, next: &Location) {
        self.line_end = next.line_start;
        self.column_end = next.column_start;
    }
}
