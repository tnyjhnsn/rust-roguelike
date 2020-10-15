use serde_json::Value;

type Particle = (i32, Vec<usize>);

pub struct Particles {
    pub particles: Vec<Particle>,
}

impl Particles {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, particle: Particle ) {
        self.particles.push(particle);
    }

    pub fn get_particles(&mut self) -> Option<Value> {
        if self.particles.len() > 0 {
            let n = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();
            let particles = serde_json::to_value((n.as_micros() as u64, &self.particles)).unwrap();
            self.particles.clear();
            Some(particles)
        } else {
            None
        }
    }
}

