use rand::Rng;

pub struct NBody {
    // Massive particles
    massive_positions: Vec<[f32; 3]>,
    massive_velocities: Vec<[f32; 3]>,
    massive_masses: Vec<f32>,

    // Ghost particles
    ghost_positions: Vec<[f32; 3]>,
    ghost_velocities: Vec<[f32; 3]>,
    ghost_kinds: Vec<f32>,
}

impl NBody {
    pub fn new() -> Self {
        Self {
            massive_positions: Vec::new(),
            massive_velocities: Vec::new(),
            massive_masses: Vec::new(),

            ghost_positions: Vec::new(),
            ghost_velocities: Vec::new(),
            ghost_kinds: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        let num_particles = self.massive_positions.len();
        let num_ghosts = self.ghost_positions.len();

        if self.massive_positions.is_empty() {
            return false;
        }
        if self.ghost_positions.is_empty() {
            return false;
        }

        // GPU buffer size must be a multiple of 64
        if num_particles % 64 != 0 {
            return false;
        }

        // GPU buffer size must be a multiple of 64
        if num_ghosts % 64 != 0 {
            return false;
        }

        // Check that the massive particle arrays are the same length
        if self.massive_positions.len() != self.massive_masses.len()
            || self.massive_velocities.len() != self.massive_masses.len()
        {
            return false;
        }

        // Check that the ghost particle arrays are the same length
        if self.ghost_positions.len() != self.ghost_kinds.len()
            || self.ghost_velocities.len() != self.ghost_kinds.len()
        {
            return false;
        }

        true
    }

    pub fn massive_positions(&self) -> &[[f32; 3]] {
        &self.massive_positions
    }

    pub fn massive_velocities(&self) -> &[[f32; 3]] {
        &self.massive_velocities
    }

    pub fn massive_masses(&self) -> &[f32] {
        &self.massive_masses
    }

    pub fn num_massive_particles(&self) -> usize {
        debug_assert!(self.is_valid());

        self.massive_positions.len()
    }

    pub fn ghost_positions(&self) -> &[[f32; 3]] {
        &self.ghost_positions
    }

    pub fn ghost_velocities(&self) -> &[[f32; 3]] {
        &self.ghost_velocities
    }

    pub fn ghost_kinds(&self) -> &[f32] {
        &self.ghost_kinds
    }

    pub fn num_ghost_particles(&self) -> usize {
        debug_assert!(self.is_valid());
        self.ghost_positions.len()
    }

    pub fn add_massive_particle(&mut self, position: [f32; 3], velocity: [f32; 3], mass: f32) {
        debug_assert!(mass > 0.0);

        self.massive_positions.push(position);
        self.massive_velocities.push(velocity);
        self.massive_masses.push(mass);
    }

    pub fn add_massive_disc(
        &mut self,
        rng: &mut impl Rng,
        grav_const: f32,
        centre: [f32; 3],
        drift: [f32; 3],
        radius: f32,
        disc_mass: f32,
        num_particles: usize,
    ) {
        debug_assert!(radius > 0.0);
        debug_assert!(num_particles > 0);

        self.massive_positions.reserve_exact(num_particles);
        self.massive_velocities.reserve_exact(num_particles);
        self.massive_masses.reserve_exact(num_particles);

        for _ in 0..num_particles {
            let r = rng.gen_range(0.0f32..1.0).sqrt() * radius;
            let theta = rng.gen_range(0.0..2.0 * std::f32::consts::PI);

            let angular_velocity = grav_const * disc_mass * r / (radius * radius);

            let dx = r * theta.cos();
            let dy = r * theta.sin();
            let position = [centre[0] + dx, centre[1] + dy, centre[2]];

            let vx = angular_velocity * theta.sin();
            let vy = angular_velocity * -theta.cos();
            let velocity = [vx + drift[0], vy + drift[1], drift[2]];

            self.massive_positions.push(position);
            self.massive_velocities.push(velocity);
            self.massive_masses.push(disc_mass / num_particles as f32);
        }
    }

    pub fn add_massive_system(
        &mut self,
        rng: &mut impl Rng,
        _grav_const: f32,
        centre: [f32; 3],
        drift: [f32; 3],
        radius: f32,
        centre_mass: f32,
        disc_mass: f32,
        num_particles: usize,
    ) {
        debug_assert!(radius > 0.0);
        debug_assert!(num_particles > 0);

        self.massive_positions.reserve_exact(num_particles + 1);
        self.massive_velocities.reserve_exact(num_particles + 1);
        self.massive_masses.reserve_exact(num_particles + 1);

        self.massive_positions.push(centre);
        self.massive_velocities.push(drift);
        self.massive_masses.push(centre_mass);

        for _ in 0..num_particles {
            let mut dx = rng.gen_range(-radius..radius);
            let mut dy = rng.gen_range(-radius..radius);

            while dx * dx + dy * dy > radius * radius {
                dx = rng.gen_range(-radius..radius);
                dy = rng.gen_range(-radius..radius);
            }

            let r = (dx * dx + dy * dy).sqrt();
            let theta = dy.atan2(dx);

            let angular_velocity = (centre_mass / r).sqrt();

            let vx = angular_velocity * theta.sin();
            let vy = angular_velocity * -theta.cos();

            let position = [centre[0] + dx, centre[1] + dy, centre[2]];
            let velocity = [vx + drift[0], vy + drift[1], drift[2]];

            self.massive_positions.push(position);
            self.massive_velocities.push(velocity);
            self.massive_masses.push(disc_mass / num_particles as f32);
        }
    }

    pub fn add_ghost_field(
        &mut self,
        rng: &mut impl Rng,
        centre: [f32; 3],
        drift: [f32; 3],
        radius: f32,
        centre_mass: f32,
        num_particles: usize,
        kind: f32,
    ) {
        debug_assert!(radius > 0.0);
        debug_assert!(num_particles > 0);

        self.ghost_positions.reserve_exact(num_particles);
        self.ghost_velocities.reserve_exact(num_particles);
        self.ghost_kinds.reserve_exact(num_particles);

        for _ in 0..num_particles {
            let r = rng.gen_range(0.0f32..1.0).sqrt() * radius;
            let theta = rng.gen_range(0.0..2.0 * std::f32::consts::PI);

            let dx = r * theta.cos();
            let dy = r * theta.sin();
            let position = [centre[0] + dx, centre[1] + dy, centre[2]];

            let angular_velocity = (centre_mass / r).sqrt();
            let vx = angular_velocity * theta.sin();
            let vy = angular_velocity * -theta.cos();

            let velocity = [vx + drift[0], vy + drift[1], drift[2]];

            self.ghost_positions.push(position);
            self.ghost_velocities.push(velocity);
            self.ghost_kinds.push(kind);
        }
    }
}
