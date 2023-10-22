use rand::Rng;

pub struct NBody {
    massive_positions: Vec<[f32; 3]>,
    massive_velocities: Vec<[f32; 3]>,
    massive_masses: Vec<f32>,
}

impl NBody {
    pub fn new() -> Self {
        Self {
            massive_positions: Vec::new(),
            massive_velocities: Vec::new(),
            massive_masses: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.massive_positions.len() == self.massive_masses.len()
            && self.massive_velocities.len() == self.massive_masses.len()
            && self.massive_positions.len() > 0
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
        self.massive_positions.len()
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

            let dx = r * theta.cos();
            let dy = r * theta.sin();

            let effective_mass = disc_mass * (r / radius).powi(3);
            let angular_velocity = (effective_mass / r).sqrt();

            let vx = angular_velocity * theta.sin();
            let vy = angular_velocity * -theta.cos();

            let position = [centre[0] + dx, centre[1] + dy, centre[2]];
            let velocity = [vx + drift[0], vy + drift[1], drift[2]];

            self.massive_positions.push(position);
            self.massive_velocities.push(velocity);
            self.massive_masses.push(disc_mass / num_particles as f32);
        }
    }

    pub fn add_massive_system(
        &mut self,
        rng: &mut impl Rng,
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
}
