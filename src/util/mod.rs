use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

pub fn gcd_big(mut a: u128, mut b: u128) -> u128 {
    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn lcm_big(a: u128, b: u128) -> u128 {
    a / gcd_big(a, b) * b
}

pub fn extended_gcd_big(a: i128, b: i128) -> (i128, (i128, i128)) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r > 0 {
        let (quotient, remainder) = (old_r / r, old_r % r);
        (old_r, r) = (r, remainder);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, (old_s, old_t))
}

// Credit where credit is due: https://math.stackexchange.com/questions/2218763/how-to-find-lcm-of-two-numbers-when-one-starts-with-an-offset
pub fn lcm_with_offset_big(a_offset: i128, a: i128, b_offset: i128, b: i128) -> (i128, i128) {
    let (gcd, (s, _t)) = extended_gcd_big(a, b);
    let offset_difference = a_offset - b_offset;
    if offset_difference % gcd != 0 {
        panic!("{a_offset}+{a}s and {b_offset}+{b}t never align");
    }

    let lcm = a / gcd * b;
    (lcm - (offset_difference * s * a / gcd - a_offset) % lcm, lcm)
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

    pub fn manhattan_distance_to(&self, other: Self) -> u64 {
        self.x().abs_diff(other.x()) + self.y().abs_diff(other.y())
    }
}

impl Add<Point2D> for Point2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<Point2D> for Point2D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<Point2D> for Point2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign<Point2D> for Point2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<i64> for Point2D {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign<i64> for Point2D {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point3D(pub i64, pub i64, pub i64);

impl Point3D {
    pub fn origin() -> Self {
        Self(0, 0, 0)
    }

    pub fn x(&self) -> i64 {
        self.0
    }

    pub fn y(&self) -> i64 {
        self.1
    }

    pub fn z(&self) -> i64 {
        self.2
    }

    pub fn xy(&self) -> Point2D {
        Point2D(self.0, self.1)
    }

    pub fn manhattan_distance_to(&self, other: Self) -> u64 {
        self.x().abs_diff(other.x()) + self.y().abs_diff(other.y()) + self.z().abs_diff(other.z())
    }
}

impl Add<Point3D> for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign<Point3D> for Point3D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign<Point3D> for Point3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<i64> for Point3D {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<i64> for Point3D {
    fn mul_assign(&mut self, rhs: i64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<i64> for Point3D {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<i64> for Point3D {
    fn div_assign(&mut self, rhs: i64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct FPoint3D(pub f64, pub f64, pub f64);

impl FPoint3D {
    pub fn origin() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn manhattan_distance_to(&self, other: Self) -> f64 {
        (self.x() - other.x()).abs() + (self.y() - other.y()).abs() + (self.z() - other.z()).abs()
    }
}

impl Add<FPoint3D> for FPoint3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign<FPoint3D> for FPoint3D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub<FPoint3D> for FPoint3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign<FPoint3D> for FPoint3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f64> for FPoint3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f64> for FPoint3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl std::fmt::Display for FPoint3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
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

    pub fn x_values(&self) -> std::ops::RangeInclusive<i64> {
        self.min_x() ..= self.max_x()
    }

    pub fn y_values(&self) -> std::ops::RangeInclusive<i64> {
        self.min_y() ..= self.max_y()
    }

    pub fn is_within_bounds(&self, point: Point2D) -> bool {
        self.min_x() <= point.x() && point.x() <= self.max_x() && self.min_y() <= point.y() && point.y() <= self.max_y()
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

    pub fn points(&self) -> Map2DPoints {
        Map2DPoints::new(self)
    }

    pub fn tiles(&self) -> Map2DTiles<'_> {
        Map2DTiles::new(self)
    }
}

impl std::fmt::Display for Map2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            let leading = String::from_iter(std::iter::repeat(self.filler_tile as char).take((row.min_x() - self.min_x()) as usize));
            let trailing = String::from_iter(std::iter::repeat(self.filler_tile as char).take((self.max_x() - row.max_x()) as usize));
            writeln!(f, "{leading}{row}{trailing}")?;
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

    fn min_x(&self) -> i64 {
        self.x_offset
    }

    fn max_x(&self) -> i64 {
        self.x_offset + self.tiles.len() as i64 - 1
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
        write!(f, "{}", String::from_utf8_lossy(&self.tiles))
    }
}

pub struct Map2DPoints {
    min_x: i64,
    max_x: i64,
    max_y: i64,
    x: i64,
    y: i64,
}

impl Map2DPoints {
    fn new(map: &Map2D) -> Self {
        Self {
            min_x: map.min_x(),
            max_x: map.max_x(),
            max_y: map.max_y(),
            x: map.min_x(),
            y: map.min_y(),
        }
    }
}

impl Iterator for Map2DPoints {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.max_y {
            None
        }
        else {
            let point = Point2D(self.x, self.y);
            self.x += 1;
            if self.x > self.max_x {
                self.x = self.min_x;
                self.y += 1;
            }
            Some(point)
        }
    }
}

pub struct Map2DTiles<'a> {
    map: &'a Map2D,
    points: Map2DPoints,
}

impl<'a> Map2DTiles<'a> {
    fn new(map: &'a Map2D) -> Self {
        Self {
            map,
            points: Map2DPoints::new(map),
        }
    }
}

impl<'a> Iterator for Map2DTiles<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|point| self.map.get_at(point))
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

    pub fn new_normalize(start: i64, end: i64) -> Self {
        Self {
            start: start.min(end),
            end: start.max(end),
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

    pub fn normalized(&self) -> Interval {
        Self::new_normalize(self.start, self.end)
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

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
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

    pub fn cardinality(&self) -> i64 {
        self.intervals.iter().map(|interval| interval.size()).sum()
    }
    
    pub fn with_offset(mut self, offset: i64) -> Self {
        for interval in &mut self.intervals {
            *interval = Interval::new(interval.start() + offset, interval.end() + offset);
        }

        self
    }

    pub fn clear(&mut self) {
        self.intervals.clear();
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

    pub fn intersect(&mut self, other: &IntervalSet) {
        let mut intersection = IntervalSet::new();

        for &interval in other.intervals() {
            intersection.apply(&self.splice_interval(interval));
        }

        *self = intersection;
    }

    pub fn subtract(&mut self, other: &IntervalSet) {
        for &interval in other.intervals() {
            self.splice_interval(interval);
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
                    if splice.end() != interval.end() {
                        intervals_to_add.push(Interval::new(splice.end(), interval.end()));
                    }
                    if interval.start() != splice.start() {
                        *interval = Interval::new(interval.start(), splice.start());
                        true
                    }
                    else {
                        false
                    }
                }
                else if interval.start() < splice.end() && splice.end() <= interval.end() {
                    // Interval starts inside splice and ends outside
                    spliced_intervals.push(Interval::new(interval.start(), splice.end()));
                    if splice.end() != interval.end() {
                        *interval = Interval::new(splice.end(), interval.end());
                        true
                    }
                    else {
                        false
                    }
                }
                else if interval.start() <= splice.start() && splice.start() < interval.end() {
                    // Interval starts outside splice and ends inside
                    spliced_intervals.push(Interval::new(splice.start(), interval.end()));
                    if interval.start() != splice.start() {
                        *interval = Interval::new(interval.start(), splice.start());
                        true
                    }
                    else {
                        false
                    }
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

impl std::fmt::Display for IntervalSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut intervals_iter = self.intervals.iter();
        if let Some(interval) = intervals_iter.next() {
            write!(f, "{interval}")?;
            for interval in intervals_iter {
                write!(f, " U {interval}")?;
            }
        }
        else {
            write!(f, "{{}}")?;
        }
        Ok(())
    }
}

pub struct EquationSystem(pub Vec<Vec<f64>>);

impl EquationSystem {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self(std::iter::repeat_with(|| std::iter::repeat(0.0).take(cols).collect()).take(rows).collect())
    }

    pub fn row_count(&self) -> usize {
        self.0.len()
    }

    pub fn col_count(&self) -> usize {
        self.0[0].len()
    }

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        for col in 0..self.col_count() {
            (self.0[row1][col], self.0[row2][col]) = (self.0[row2][col], self.0[row1][col]);
        }
    }

    pub fn mul_row(&mut self, row: usize, value: f64) {
        for element in &mut self.0[row] {
            *element *= value;
        }
    }

    pub fn div_row(&mut self, row: usize, value: f64) {
        for element in &mut self.0[row] {
            *element /= value;
        }
    }

    pub fn add_row(&mut self, row: usize, from: usize, mul: f64) {
        for col in 0..self.col_count() {
            self.0[row][col] += self.0[from][col] * mul;
        }
    }

    pub fn solve_rref(&mut self) {
        let mut target_row: usize = 0;
        for col in 0..self.col_count() {
            if target_row >= self.row_count() {
                break;
            }
            for row in target_row..self.row_count() {
                if self.0[row][col] != 0.0 {
                    self.swap_rows(row, target_row);
                    self.div_row(target_row, self.0[target_row][col]);
                    for cancel_row in 0..self.row_count() {
                        if cancel_row != target_row && self.0[cancel_row][col] != 0.0 {
                            self.add_row(cancel_row, target_row, -self.0[cancel_row][col]);
                        }
                    }
                    target_row += 1;
                    break;
                }
            }
        }
    }
}
