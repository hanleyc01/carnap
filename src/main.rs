
mod source_filter;
mod waves;
mod plot;

use crate::waves::*;
use crate::source_filter::*;
use crate::plot::*;

// +++

/// Test which creates and filters a source wave, then displays it onto a png
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    Chart::init_chart_wave(
        0.0,
        10.0,
        0.003,
        SimpleWave::sine_wave(10.0, 0.0),
        0.0,
        10.0,
        -20.0,
        20.0
    )?;

    Ok(())
} 
