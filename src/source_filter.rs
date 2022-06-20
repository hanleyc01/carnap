
use crate::waves::*;

// +++ Physiological
/// Filter
pub struct Filter {
    pub pharynx: Cavity,
    pub oral: Cavity,
    pub round: Cavity,
}

impl Filter {

    pub fn new(pharynx: Cavity, oral: Cavity, round: Cavity) -> Self {
       Self { pharynx, oral, round } 
    }

    pub fn formants(&self, source_wave: Source) -> Vec<SimpleWave> {
        let mut result1: Vec<SimpleWave> = Vec::new();
        
        for harmonic in source_wave.complex_wave {
            let f1 = harmonic.clone();
            result1.push(self.pharynx.amp_and_damp(f1));
        }
        
        let mut result2: Vec<SimpleWave> = Vec::new();
        for harmonic in result1 {
            let f2 = harmonic.clone();
            result2.push(self.oral.amp_and_damp(f2));
        }

        let mut result3: Vec<SimpleWave> = Vec::new();
        for harmonic in result2 {
            let f3 = harmonic.clone();
            result3.push(self.round.amp_and_damp(f3));
        }
        
        result3
        
    }
}

/// The distinct filters within the overall Filter (e.g., pharyngeal cavity,
/// oral cavity, and lip roundedness)
pub struct Cavity {
    pub resonance_freq: f64,
    pub phase: Option<f64>,
}

impl Cavity {
    
    /// Constructs a cavity given area, volume, and length of the cavity
    pub fn new(area: f64, volume: f64, length: f64, phase: f64) -> Self {
        let v_sound = 343f64;
        let v_div_2pi = v_sound / 2f64 * std::f64::consts::PI;
        let sqrt_a_div_vl = (area / volume * length).sqrt();
        let f_res = v_div_2pi * sqrt_a_div_vl;
        Self { 
            resonance_freq: f_res,
            phase: {
                if phase == 0.0 {
                    None
                } else {
                    Some(phase)
                }
            }
        }
    }

    /// Constructor from resonance_frequency
    pub fn from_freq(resonance_freq: f64, phase: Option<f64>) -> Self {
        Self { resonance_freq, phase }
    }

    /// Amplify and dampen harmonics
    pub fn amp_and_damp(&self, mut harmonic: SimpleWave) -> SimpleWave {
        match self.phase {
            Some(p) => {
                harmonic.amplitude = 2. * harmonic.amplitude * (harmonic.phase - p / 2.).cos();
                harmonic
            },
            None => {
                harmonic.amplitude = 2. * harmonic.amplitude * (harmonic.phase / 2.).cos();
                harmonic
            },
        }
    }

}


/// Source in our source-filter theory of speech;
/// This is the primary abstraction for the vocal folds vibration.
#[derive(Clone)]
pub struct Source {
   pub complex_wave: Vec<SimpleWave>, 
}

