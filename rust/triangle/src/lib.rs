use itertools::Itertools;

pub struct Triangle {
    sides: [u64; 3],
}

fn side_windows_2(sides: &[u64; 3]) -> impl Iterator<Item = (&u64, &u64)> {
    sides.iter().cycle().tuple_windows().take(3)
}

fn side_windows_3(sides: &[u64; 3]) -> impl Iterator<Item = (&u64, &u64, &u64)> {
    sides.iter().cycle().tuple_windows().take(3)
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|x| *x == 0) {
            None
        } else {
            if side_windows_3(&sides).any(|(x, y, z)| x + y < *z) {
                None
            } else {
                Some(Triangle { sides })
            }
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|x| *x == self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        side_windows_2(&self.sides).all(|(x, y)| x != y)
    }

    pub fn is_isosceles(&self) -> bool {
        side_windows_2(&self.sides).any(|(x, y)| x == y)
    }
}
