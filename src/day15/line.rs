use super::sensor::Position;

pub struct Line {
    k: i32,
    m: i32,
    x_min: i32,
    x_max: i32,
}

impl Line {
    pub fn new(km: (i32, i32), x_range: (i32, i32)) -> Line {
        Line {
            k: km.0,
            m: km.1,
            x_min: x_range.0,
            x_max: x_range.1,
        }
    }

    pub fn perpendicular(&self, other: &Line) -> bool {
        if self.k == other.k
            && self.m == other.m
            && self.x_min <= other.x_max
            && other.x_min <= self.x_max
        {
            return true;
        }
        false
    }

    pub fn intersection(&self, other: &Line) -> Option<Position> {
        if (self.k == other.k && self.m != other.m) {
            None
        } else {
            let x = (other.m - self.m) / (self.k - other.k);
            let y = self.k * x + self.m;
            Some(Position { x, y })
        }
    }
}
