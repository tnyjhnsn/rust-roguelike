use serde_json::Value;

type Particle = (i32, Vec<usize>);

pub struct Particles {
    pub particles: Vec<Particle>,
    pub has_particles: bool,
}

impl Particles {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            has_particles: false,
        }
    }

    pub fn add_particle(&mut self, particle: Particle ) {
        self.particles.push(particle);
        self.has_particles = true;
    }

    pub fn get_particles(&mut self) -> Option<Value> {
        if self.has_particles {
            let n = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
            let particles = serde_json::to_value((n.as_micros() as u64, &self.particles)).unwrap();
            self.particles.clear();
            self.has_particles = false;
            Some(particles)
        } else {
            None
        }
    }
}

