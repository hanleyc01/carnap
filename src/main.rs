
use plotters::prelude::*;

// +++ Physiological
/// Filter
struct Filter {
    pharynx: Cavity,
    oral: Cavity,
    round: Cavity,
}

/// The distinct filters within the overall Filter (e.g., pharyngeal cavity,
/// oral cavity, and lip roundedness)
struct Cavity {
    resonance_freq: Vec<f32>,
}

/// Source in our source-filter theory of speech;
/// This is the primary abstraction for the vocal folds vibration.
struct Source {
   complex_wave: Vec<SimpleWave>, 
}
// +++

// +++ Physical

/// Abstraction representing a simple sinusoidal wave
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
            angular_frequency: 2. *  std::f64::consts::PI * frequency,
            phase,
        }
    }

    /// Generates a default wave with a frequency of 440.0 Hz and an amplitude
    /// of 0.2 Pa
    fn default() -> Self {
        Self {
            amplitude: 0.2,
            period: 1.0 / 440.0,
            frequency: 440.0,
            angular_frequency: 2.0 * std::f64::consts::PI * 440.0,
            phase: 0.0,
        }
    }

    /// Calculate displacement of height over some time period, range of [init_time, fin_time)
    fn displace_y(&self, init_time: f64, fin_time: f64, step: f64) -> Vec<f64> {
        let mut displacement: Vec<f64> = Vec::new();
        let range = get_range(init_time, fin_time, step);
        for t in range {
            let body = self.angular_frequency * t + self.phase;
            displacement.push(self.amplitude * body.sin());
        }

        displacement
    }

}

fn get_range(init_time: f64, fin_time: f64, step: f64) -> Vec<f64> {
    let mut to_step = init_time;
    let mut range: Vec<f64> = Vec::new();
    while to_step <= fin_time {
        range.push(to_step);
        to_step += step;
    }

    range
}

// +++

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test simple wave
    let test_wave: SimpleWave = SimpleWave::default();
    let init_time = 0.4;
    let fin_time = 1.;
    let step = 0.03;
    let test_displace = test_wave.displace_y(init_time, fin_time, step);
    let range = get_range(init_time, fin_time, step);

    println!("{:#?} {:#?}", &test_displace, &range);
        
    println!("Range generated!");

    // The chart
    let root = BitMapBackend::new("1.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Simple Periodic Wave", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(500)
        .y_label_area_size(500)
        .build_cartesian_2d(test_displace[0]..test_displace[test_displace.len()-1], range[0]..range[range.len()-1])?;
  
    chart.configure_mesh().draw()?;

    println!("range generated");

    chart
        .draw_series(LineSeries::new(std::iter::zip(test_displace, range), &RED))?
        .label("Sample Sinusoidal Wave");
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    println!("Chart drawn!");

    root.present()?;

    Ok(())
} 
