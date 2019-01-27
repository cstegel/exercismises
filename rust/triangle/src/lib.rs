use itertools::Itertools;

pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|x| *x == 0) {
            None
        } else {
            if sides
                .iter()
                .cycle()
                .tuple_windows()
                .take(3)
                .any(|(x, y, z)| x + y < *z)
            {
                None
            } else {
                Some(Triangle { sides })
            }
        }
    }

    // fn side_combos<'a>(&'a self) -> impl Iterator<Item = u64> {
    //     // impl Iterator<Item = <std::slice::Iter<'_, u64> as Iterator>::Item>
    //     // itertools::tuple_impl::TupleWindows<std::slice::Iter<'_, u64>, _>
    //     // self.sides.iter().cycle().tuple_windows().take(3)
    //     self.sides.iter().tuple_windows()
    // }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|x| *x == self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        self.sides
            .iter()
            .cycle()
            .tuple_windows()
            .take(3)
            .all(|(x, y)| x != y)
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides
            .iter()
            .cycle()
            .tuple_windows()
            .take(3)
            .any(|(x, y)| x == y)
    }
}
