use aoc_utils::cartography::Point3D;

/// A particle existing in three-dimensional space. Each particle has a three-dimensional location,
/// velocity and acceleration.
#[derive(Copy, Clone, PartialEq)]
pub struct Particle3D {
    loc: Point3D,
    vel: Point3D,
    acc: Point3D,
    loc_abs: f64,
    vel_abs: f64,
    acc_abs: f64,
}

impl Particle3D {
    pub fn new(loc: &Point3D, vel: &Point3D, acc: &Point3D) -> Self {
        Self {
            loc: *loc,
            vel: *vel,
            acc: *acc,
            loc_abs: loc.get_absolute_value(),
            vel_abs: vel.get_absolute_value(),
            acc_abs: acc.get_absolute_value(),
        }
    }

    /// Updates the velocity and location of the particle.
    pub fn tick(&mut self) {
        // Update velocity by acceleration
        self.vel.shift(self.acc.x(), self.acc.y(), self.acc.z());
        // Update location by velocity
        self.loc.shift(self.vel.x(), self.vel.y(), self.vel.z());
    }
}

impl PartialOrd for Particle3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Compare absolute acceleration
        match self.acc_abs.partial_cmp(&other.acc_abs) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        // Compare absolute velocity
        match self.vel_abs.partial_cmp(&other.vel_abs) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        // Compare absolute location
        self.loc_abs.partial_cmp(&other.loc_abs)
    }
}
