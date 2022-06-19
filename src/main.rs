
use plotters::prelude::*;

// +++ Physiological
/// Filter
struct Filter {
    pharynx: Cavity,
    oral: Cavity,
    round: Cavity,
}

impl Filter {

    fn new(pharynx: Cavity, oral: Cavity, round: Cavity) -> Self {
       Self { pharynx, oral, round } 
    }

    fn formants(&self, source_wave: Source) -> Vec<SimpleWave> {
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
struct Cavity {
    resonance_freq: f64,
    phase: Option<f64>,
}

impl Cavity {
    
    /// Constructs a cavity given area, volume, and length of the cavity
    fn new(area: f64, volume: f64, length: f64, phase: f64) -> Self {
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

    /// Amplify and dampen harmonics
    fn amp_and_damp(&self, mut harmonic: SimpleWave) -> SimpleWave {
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
struct Source {
   complex_wave: Vec<SimpleWave>, 
}
// +++

// +++ Physical

/// Abstraction representing a simple sinusoidal wave
#[derive(Clone)]
struct SimpleWave {
    amplitude: f64,
    period: f64, 
    frequency: f64,
    angular_frequency: f64,
    phase: f64,
}

impl SimpleWave {
   
    /// Derive the simple wave given amplitude, phase, and period
    fn from_period(amplitude: f64, period: f64, phase: f64) -> Self {
        Self {
            amplitude,
            period,
            frequency: 1. / period,
            angular_frequency: 2. * std::f64::consts::PI * ( 1. / period ),
            phase,
        }

    }

    /// Derive from the simple wave given amplitude, frequency, and phase
    fn from_freq(amplitude: f64, frequency: f64, phase: f64) -> Self {
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
    fn default() -> Self {
        Self {
            amplitude: 4.,
            period: 1.0 / 440.0,
            frequency: 440.0,
            angular_frequency: 2.0 * std::f64::consts::PI * 440.0,
            phase: 0.0,
        }
    }

    fn sine_wave(amplitude: f64, phase: f64) -> Self {
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

    /// Calculate displacement of height over some time period, range of [init_time, fin_time)
    fn displace_y(&self, init_time: f64, fin_time: f64, step: f64) -> Vec<f64> {
        use std::f64::consts::PI;

        let mut displacement: Vec<f64> = Vec::new();
        let range = get_range(init_time, fin_time, step);
        for t in range {
            let body = 2.0 * PI * self.frequency * t + self.phase;
            displacement.push(self.amplitude * body.sin());
        }

        displacement
    }

}

/// Generates a range of time points given a specific step
fn get_range(init_time: f64, fin_time: f64, step: f64) -> Vec<f64> {
    let mut to_step = init_time;
    let mut range: Vec<f64> = Vec::new();
    while to_step <= fin_time {
        range.push(to_step);
        to_step += step;
    }

    range
}

/// Fourier synthesis of several simple waves into a complex wave
fn fourier_synthesis(init_time: f64, fin_time: f64, step: f64, vect: Vec<SimpleWave>) -> Vec<f64> {
    let mut resultant: Vec<f64> = Vec::new();
    let displacements:Vec<Vec<f64>> = vect.iter().map(|wave| wave.displace_y(init_time, fin_time, step)).collect();
    
    for j in 0..displacements[0].len() {
        let mut r_sum: f64 = 0.0;
        for i in 0..displacements.len() {
            r_sum += displacements[i][j];
        }
        resultant.push(r_sum);
    }

    resultant
}

// +++

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test simple wave
    let test_wave1: SimpleWave = SimpleWave::sine_wave(2., 0.);
    let test_wave2: SimpleWave = SimpleWave::default();
    let test_wave3: SimpleWave = SimpleWave::sine_wave(3.22, 1.);

    let init_time = 0.0;
    let fin_time = 15.;
    let step = 0.001;
    
    let fourier = fourier_synthesis(init_time, fin_time, step, vec![test_wave1, test_wave2, test_wave3]);

    let range = get_range(init_time, fin_time, step);
        
    println!("wave generated!");

    // The chart
    let root = BitMapBackend::new("2.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Complex Periodic Wave", ("sans-serif", 50).into_font())
        .margin(5u32)
        .x_label_area_size(50u32)
        .y_label_area_size(50u32)
        .build_cartesian_2d(init_time..fin_time, -5.000..5.3000)?;
  
    chart.configure_mesh().draw()?;

    println!("wave drawn");

    chart
        .draw_series(LineSeries::new(std::iter::zip(range, fourier), &RED))?
        .label("Wave");
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    println!("chart drawn");

    root.present()?;

    Ok(())
} 
