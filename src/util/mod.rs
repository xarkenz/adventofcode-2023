pub fn get_input(name: &'static str) -> std::io::BufReader<std::fs::File> {
    std::io::BufReader::new(std::fs::File::open(format!("./src/input/{name}"))
        .expect("unable to open input file"))
}

pub fn expect_line(result: std::io::Result<String>) -> String {
    result.expect("error while reading input file")
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point2D(pub i64, pub i64);

impl Point2D {
    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// It's honestly surprising how often you need something like this for AOC...
#[derive(Clone)]
pub struct Map2D {
    filler_tile: u8,
    rows: Vec<Map2DRow>,
    y_offset: i64,
    min_x: i64,
    max_x: i64,
}

impl Map2D {
    pub fn new(filler_tile: u8) -> Self {
        Self {
            filler_tile,
            rows: Vec::new(),
            y_offset: 0,
            min_x: 0,
            max_x: -1,
        }
    }

    pub fn from_rows(rows: impl Iterator<Item = Vec<u8>>, filler_tile: u8) -> Self {
        let mut max_x = -1;
        let rows = rows.map(|tiles| {
            max_x = max_x.max(tiles.len() as i64 - 1);
            Map2DRow::from(tiles)
        }).collect();
        Self {
            filler_tile,
            rows,
            y_offset: 0,
            min_x: 0,
            max_x,
        }
    }

    pub fn min_x(&self) -> i64 {
        self.min_x
    }

    pub fn max_x(&self) -> i64 {
        self.max_x
    }

    pub fn min_y(&self) -> i64 {
        self.y_offset
    }

    pub fn max_y(&self) -> i64 {
        self.y_offset + self.rows.len() as i64 - 1
    }

    pub fn get(&self, x: i64, y: i64) -> u8 {
        self.rows.get((y - self.y_offset) as usize)
            .map(|row| row.get(x, self.filler_tile))
            .unwrap_or(self.filler_tile)
    }

    pub fn get_at(&self, point: Point2D) -> u8 {
        let Point2D(x, y) = point;
        self.get(x, y)
    }

    pub fn put(&mut self, x: i64, y: i64, tile: u8) -> u8 {
        if self.rows.is_empty() {
            self.min_x = x;
            self.max_x = x;
            self.y_offset = y;
        }
        else {
            self.min_x = self.min_x.min(x);
            self.max_x = self.max_x.max(x);
            if y < self.y_offset {
                let filler_row_iter = std::iter::repeat(Map2DRow::new())
                    .take((self.y_offset - y) as usize);
                let mut adjusted_rows = Vec::from_iter(filler_row_iter);
                adjusted_rows.append(&mut self.rows);
                self.rows = adjusted_rows;
                self.y_offset = y;
            }
        }
        if y >= self.y_offset + self.rows.len() as i64 {
            let filler_row_iter = std::iter::repeat(Map2DRow::new())
                .take((y - self.y_offset) as usize - self.rows.len() + 1);
            self.rows.extend(filler_row_iter);
        }
        self.rows[(y - self.y_offset) as usize].put(x, tile, self.filler_tile)
    }
    
    pub fn put_at(&mut self, point: Point2D, tile: u8) -> u8 {
        let Point2D(x, y) = point;
        self.put(x, y, tile)
    }

    pub fn clear(&mut self) {
        self.rows.clear();
        self.y_offset = 0;
    }
}

impl std::fmt::Display for Map2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.y_offset {
            writeln!(f)?;
        }
        for row in self.rows.iter() {
            writeln!(f, "{row}")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Map2DRow {
    tiles: Vec<u8>,
    x_offset: i64,
}

impl Map2DRow {
    fn new() -> Self {
        Self {
            tiles: Vec::new(),
            x_offset: 0,
        }
    }

    fn get(&self, x: i64, filler_tile: u8) -> u8 {
        self.tiles.get((x - self.x_offset) as usize)
            .copied()
            .unwrap_or(filler_tile)
    }

    fn put(&mut self, x: i64, tile: u8, filler_tile: u8) -> u8 {
        if self.tiles.is_empty() {
            self.x_offset = x;
        }
        else if x < self.x_offset {
            let filler_tile_iter = std::iter::repeat(filler_tile)
                .take((self.x_offset - x) as usize);
            let mut adjusted_content = Vec::from_iter(filler_tile_iter);
            adjusted_content.append(&mut self.tiles);
            self.tiles = adjusted_content;
            self.x_offset = x;
        }
        if x >= self.x_offset + self.tiles.len() as i64 {
            let filler_tile_iter = std::iter::repeat(filler_tile)
                .take((x - self.x_offset) as usize - self.tiles.len() + 1);
            self.tiles.extend(filler_tile_iter);
        }
        std::mem::replace(&mut self.tiles[(x - self.x_offset) as usize], tile)
    }
}

impl From<Vec<u8>> for Map2DRow {
    fn from(tiles: Vec<u8>) -> Self {
        Self {
            tiles,
            x_offset: 0,
        }
    }
}

impl std::fmt::Display for Map2DRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{: >2$}{}", "", String::from_utf8_lossy(&self.tiles), self.x_offset as usize)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    pub fn new(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.end
    }

    pub fn size(&self) -> i64 {
        self.end - self.start
    }

    pub fn contains(&self, value: i64) -> bool {
        self.start() <= value && value < self.end()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.start() <= other.end() && self.end() >= other.start()
    }

    pub fn merge(&self, other: &Self) -> Option<Interval> {
        if self.intersects(other) {
            Some(Interval::new(
                self.start().min(other.start()),
                self.end().max(other.end()),
            ))
        }
        else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntervalSet {
    intervals: Vec<Interval>,
}

impl IntervalSet {
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }

    pub fn intervals(&self) -> &[Interval] {
        &self.intervals
    }
    
    pub fn with_offset(mut self, offset: i64) -> Self {
        for interval in &mut self.intervals {
            *interval = Interval::new(interval.start() + offset, interval.end() + offset);
        }

        self
    }
    
    pub fn apply(&mut self, other: &IntervalSet) {
        for &interval in other.intervals() {
            self.apply_interval(interval);
        }
    }

    pub fn apply_interval(&mut self, interval: Interval) {
        if interval.size() > 0 {
            let mut merged = interval;
            self.intervals.retain(|interval| {
                if let Some(merged_interval) = merged.merge(interval) {
                    merged = merged_interval;
                    false
                }
                else {
                    true
                }
            });
            self.intervals.push(merged);
        }
    }

    pub fn splice_interval(&mut self, splice: Interval) -> IntervalSet {
        let mut spliced_intervals = Vec::new();

        if splice.size() > 0 {
            let mut intervals_to_add = Vec::new();

            self.intervals.retain_mut(|interval| {
                if splice.start() <= interval.start() && interval.end() <= splice.end() {
                    // Interval is fully contained within splice
                    spliced_intervals.push(*interval);
                    false
                }
                else if interval.start() <= splice.start() && splice.end() <= interval.end() {
                    // Interval fully contains splice
                    spliced_intervals.push(splice);
                    intervals_to_add.push(Interval::new(splice.end(), interval.end()));
                    *interval = Interval::new(interval.start(), splice.start());
                    true
                }
                else if interval.start() < splice.end() && splice.end() <= interval.end() {
                    // Interval starts inside splice and ends outside
                    spliced_intervals.push(Interval::new(interval.start(), splice.end()));
                    *interval = Interval::new(splice.end(), interval.end());
                    true
                }
                else if interval.start() <= splice.start() && splice.start() < interval.end() {
                    // Interval starts outside splice and ends inside
                    spliced_intervals.push(Interval::new(splice.start(), interval.end()));
                    *interval = Interval::new(interval.start(), splice.start());
                    true
                }
                else {
                    // Interval is unaffected by splice
                    true
                }
            });

            self.intervals.append(&mut intervals_to_add);
        }
        
        IntervalSet {
            intervals: spliced_intervals,
        }
    }
}
