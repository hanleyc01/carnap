
/// Abstraction representing a simple sinusoidal wave
#[derive(Clone)]
pub struct SimpleWave {
    pub amplitude: f64,
    pub period: f64, 
    pub frequency: f64,
    pub angular_frequency: f64,
    pub phase: f64,
}

impl SimpleWave {
   
    /// Derive the simple wave given amplitude, phase, and period
    pub fn from_period(amplitude: f64, period: f64, phase: f64) -> Self {
        Self {
            amplitude,
            period,
            frequency: 1. / period,
            angular_frequency: 2. * std::f64::consts::PI * ( 1. / period ),
            phase,
        }

    }

    /// Derive from the simple wave given amplitude, frequency, and phase
    pub fn from_freq(amplitude: f64, frequency: f64, phase: f64) -> Self {
        Self {
            amplitude,
            period: 1. / frequency,
            frequency,
            angular_frequency: 2. * std::f64::consts::PI * frequency,
            phase,
        }
    }

    /// Generates a default wave with a frequency of 440.0 Hz and an amplitude
    /// of 0.2 Pa
    pub fn default() -> Self {
        Self {
            amplitude: 4.,
            period: 1.0 / 440.0,
            frequency: 440.0,
            angular_frequency: 2.0 * std::f64::consts::PI * 440.0,
            phase: 0.0,
        }
    }

    pub fn sine_wave(amplitude: f64, phase: f64) -> Self {
       use std::f64::consts::PI;
       let period = 2.0 * PI;
       let frequency = 1.0 / period;
       let angular_frequency = 2.0 * PI * frequency;
       Self {
            amplitude,
            period,
            frequency,
            angular_frequency,
            phase
       }
    }

    /// Calculate displacement of height over some time period, range of range [init_time, fin_time)
    pub fn displace_y(&self, range: &Vec<f64>) -> Vec<f64> {
        use std::f64::consts::PI;

        let mut displacement: Vec<f64> = Vec::new();
        for t in range {
            let body = 2.0 * PI * self.frequency * t + self.phase;
            displacement.push(self.amplitude * body.sin());
        }

        displacement
    }

}

/// Generates a range of time points given a specific step
pub fn get_range(init_time: f64, fin_time: f64, step: f64) -> Vec<f64> {
    let mut range: Vec<f64> = Vec::new();

    let mut t = init_time;
    while t <= fin_time {
        range.push(t);
        t += step;
    }

    range
}

/// Fourier synthesis of several simple waves into a complex wave
pub fn fourier_synthesis(range: &Vec<f64>, vect: Vec<SimpleWave>) -> Vec<(f64, f64)> {
    let mut resultant: Vec<f64> = Vec::new();
    let displacements:Vec<Vec<f64>> = vect.iter().map(|wave| wave.displace_y(range)).collect();
    
    for j in 0..displacements[0].len() {
        
        let mut resultant_sum = 0.;
        for i in 0..displacements.len() {
            
            resultant_sum += displacements[i][j];


        }

        resultant.push(resultant_sum);
    
        assert_eq!(displacements[0][j] + displacements[1][j] + displacements[2][j], resultant[j]);
    } 
    
    std::iter::zip(range.clone(), resultant).collect()
}


