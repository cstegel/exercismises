use itertools::Itertools;

use std::cmp::PartialOrd;

use num::Num;

pub struct Triangle<T> {
    sides: [T; 3],
}

fn side_windows_2<T>(sides: &[T; 3]) -> impl Iterator<Item = (&T, &T)> {
    sides.iter().cycle().tuple_windows().take(3)
}

fn side_windows_3<T>(sides: &[T; 3]) -> impl Iterator<Item = (&T, &T, &T)> {
    sides.iter().cycle().tuple_windows().take(3)
}

impl<T> Triangle<T>
where
    T: Num + PartialOrd + From<i32> + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|x| *x == T::from(0)) {
            None
        } else {
            if side_windows_3(&sides).any(|(x, y, z)| *x + *y < *z) {
                None
            } else {
                Some(Triangle { sides })
            }
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|x| *x == self.sides[0])
    }

    // TODO: how would we pad comparisons by float epsilons
    // for float but be exact for int types?
    pub fn is_scalene(&self) -> bool {
        side_windows_2(&self.sides).all(|(x, y)| x != y)
    }

    pub fn is_isosceles(&self) -> bool {
        side_windows_2(&self.sides).any(|(x, y)| x == y)
    }
}
