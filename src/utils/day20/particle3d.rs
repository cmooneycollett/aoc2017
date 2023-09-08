use aoc_utils::cartography::Point3D;

/// A particle existing in three-dimensional space. Each particle has a three-dimensional location,
/// velocity and acceleration.
#[derive(Copy, Clone, PartialEq)]
pub struct Particle3D {
    loc: Point3D,
    vel: Point3D,
    acc: Point3D,
    loc_manh: u64,
    vel_manh: u64,
    acc_manh: u64,
}

impl Particle3D {
    pub fn new(loc: &Point3D, vel: &Point3D, acc: &Point3D) -> Self {
        Self {
            loc: *loc,
            vel: *vel,
            acc: *acc,
            loc_manh: loc.get_manhattan_distance_origin(),
            vel_manh: vel.get_manhattan_distance_origin(),
            acc_manh: acc.get_manhattan_distance_origin(),
        }
    }

    /// Updates the velocity and location of the particle.
    pub fn tick(&mut self) {
        // Update velocity by acceleration
        self.vel.shift(self.acc.x(), self.acc.y(), self.acc.z());
        // Update location by velocity
        self.loc.shift(self.vel.x(), self.vel.y(), self.vel.z());
    }

    /// Returns the value of the "loc" field.
    pub fn loc(&self) -> &Point3D {
        &self.loc
    }
}

impl PartialOrd for Particle3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Compare absolute acceleration
        match self.acc_manh.partial_cmp(&other.acc_manh) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        // Compare absolute velocity
        match self.vel_manh.partial_cmp(&other.vel_manh) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        // Compare absolute location
        self.loc_manh.partial_cmp(&other.loc_manh)
    }
}
