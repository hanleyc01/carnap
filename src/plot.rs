use plotters::prelude::*;
use crate::waves::*;

/// Main data-abstraction behind the actual graphical display
/// of the wave
pub struct Chart {
    pub init_time: f64,
    pub final_time: f64,
    pub step: f64,
    pub wave: Vec<SimpleWave>,
}

impl Chart {
    
    /// Encapsulated initialization of the chart taking certain parameters.
    pub fn init_chart_wave(init_time: f64, final_time: f64, step: f64,
                           wave: SimpleWave, x_init: f64, x_final: f64, y_init: f64, y_final: f64 ) 
        -> Result<(), Box<dyn std::error::Error>> 
    {
        let range = get_range(init_time, final_time, step);
        let wave_displacement = wave.displace_y(&range);
        
        // The chart
        let root = BitMapBackend::new("source_filter_simulation.png", (640, 480)).into_drawing_area();
            root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Complex Periodic Wave", ("sans-serif", 50).into_font())
            .margin(5u32)
            .x_label_area_size(50u32)
            .y_label_area_size(50u32)
            .build_cartesian_2d(x_init..x_final, y_init..y_final)?;
            //.build_cartesian_2d(10.0..20.0, -20.00..20.00)?;
 
        chart.configure_mesh().draw()?;

        println!("wave drawn");

        chart
            .draw_series(LineSeries::new(std::iter::zip(range, wave_displacement),&RED))?
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
}
