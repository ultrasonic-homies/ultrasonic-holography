use crate::hat::Point;

const MAX_ACCEL: f32 = 1.0; // m/s^2
const MAX_VEL: f32 = 1.0; // m/s

pub struct Particle {
    pub point: Point,
    pub t_sep: f32,
    pub vel: Point, // using Point as a velocity vector
}

impl Particle {
    pub fn new(point: Point, t_sep: f32) -> Self {
        Particle {
            point,
            t_sep,
            vel: Point::new(0.0, 0.0, 0.0),
        }
    }

    // generate a string of control points to move from self.point to point
    pub fn move_to(&mut self, point: Point, speed: f32) -> Vec<Point> {
        let dr = self.t_sep * speed;
        let r = point - self.point;
        let norm = r.norm();
        let rhat = r / r.norm();

        // get evenly spaced points
        let n = (norm / dr).ceil() as usize;
        let dr = norm / n as f32;

        let out = (1..n + 1)
            .map(|i| self.point + i as f32 * dr * rhat)
            .collect();
        self.point = point;
        out
    }

    // TODO: functions that take into account max acceleration/velocity, can make smooth curves, etc.
}

#[cfg(test)]
mod tests {
    use crate::hat::Point;

    use super::Particle;

    #[test]
    fn test_move_to() {
        let mut p = Particle::new(Point::new(1.0, 2.0, 2.0), 0.1);
        let dest = Point::new(2.0, 2.0, 1.0);

        let cps = p.move_to(dest, 1.0);
        assert!(cps.len() == 15);
        assert_eq!(*cps.last().unwrap(), dest);
        println!("{:?}", cps);
    }
}
