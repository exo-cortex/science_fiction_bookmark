use std::fs::File;
// use std::io::{BufWriter, Write};

mod svg;

fn lower_tick(file: &mut File, xpos: f64, ypos: f64, length: f64, thickness: f64, tick_name: &str) {
    svg::line(
        file,
        xpos,
        ypos,
        xpos,
        ypos + length,
        thickness,
        "black",
        &svg::StrokeCapType::Square,
    );
    svg::text_start_aligned(file, tick_name, xpos, ypos + length + 10.0, 45);
}

fn upper_tick(file: &mut File, xpos: f64, ypos: f64, length: f64, thickness: f64, tick_name: &str) {
    svg::line(
        file,
        xpos,
        ypos,
        xpos,
        ypos - length,
        thickness,
        "black",
        &svg::StrokeCapType::Square,
    );
    svg::text_start_aligned(file, tick_name, xpos, ypos - length - 10.0, -45);
}

fn lower_blind_tick(file: &mut File, xpos: f64, ypos: f64, length: f64, thickness: f64) {
    svg::line(
        file,
        xpos,
        ypos,
        xpos,
        ypos + length,
        thickness,
        "black",
        &svg::StrokeCapType::Square,
    );
}

fn upper_blind_tick(file: &mut File, xpos: f64, ypos: f64, length: f64, thickness: f64) {
    svg::line(
        file,
        xpos,
        ypos,
        xpos,
        ypos - length,
        thickness,
        "black",
        &svg::StrokeCapType::Square,
    );
}

const C: f64 = 299_792_458.0;

fn main() -> std::io::Result<()> {
    let upper_ticks: Vec<(&str, f64)> = vec![
        ("1 mm/c", 0.001 / C),
        ("1 in/c", 0.0254 / C),
        ("1 cm/c", 0.01 / C),
        ("1 m/c", 1.0 / C),
        ("1 km/c", 1000.0 / C),
        ("1 mile/c", 1609.344 / C),
        ("1 second", 1.0),
        ("1 minute", 60.0),
        ("1 AU/c", 149_597_870_700.0 / C),
        ("1 hour", 3600.0),
        ("1 day", 3600.0 * 24.0),
        ("1 week", 3600.0 * 24.0 * 7.0),
        ("1 month", 3600.0 * 24.0 * 31.0),
        ("1 year", 3600.0 * 24.0 * 365.0),
        ("1 decade", 3600.0 * 24.0 * 365.0 * 10.0),
        ("30 years", 3600.0 * 24.0 * 365.0 * 30.0),
        ("1 century", 3600.0 * 24.0 * 365.0 * 100.0),
        ("1 millenium", 3600.0 * 24.0 * 365.0 * 1000.0),
    ];

    let mut upper_blind_ticks: Vec<f64> = vec![
        3600.0 * 24.0 * 2.0,
        3600.0 * 24.0 * 3.0,
        3600.0 * 24.0 * 4.0,
        3600.0 * 24.0 * 5.0,
        3600.0 * 24.0 * 6.0,
    ];

    for i in 1..4 {
        upper_blind_ticks.push(i as f64 * 15.0);
    }
    for i in 1..4 {
        upper_blind_ticks.push(i as f64 * 60.0 * 15.0);
    }
    for i in 2..24 {
        upper_blind_ticks.push(i as f64 * 60.0 * 60.0);
    }
    for i in 2..31 {
        upper_blind_ticks.push(i as f64 * 60.0 * 60.0 * 24.0);
    }

    let lower_ticks: Vec<(&str, f64)> = vec![
        ("1 picosecond", 0.000_000_000_001),
        ("1 nanosecond", 0.000_000_001),
        ("1 microsecond", 0.000_001),
        ("1 millisecond", 0.001),
        ("1 second", 1.0),
        ("100 seconds", 100.0),
        ("1 kilosecond", 1000.0),
        ("1 megasecond", 1000_000.0),
        ("1 gigasecond", 1000_000_000.0),
        ("1 terasecond", 1000_000_000_000.0),
    ];

    let lower_blind_ticks: Vec<f64> = vec![
        0.000_000_000_01,
        0.000_000_000_1,
        0.000_000_01,
        0.000_000_1,
        0.000_01,
        0.000_1,
        0.01,
        0.1,
        10.0,
        100.0,
        10_000.0,
        100_000.0,
        10_000_000.0,
        100_000_000.0,
        10_000_000_000.0,
        100_000_000_000.0,
    ];

    let mut file = File::create("output.svg").expect("could not open file.");

    svg::header(&mut file, 2000.0, 240.0);
    svg::set_text_style(&mut file, 8);

    let (x_offset, y_offset) = (1000.0, 120.0);
    let x_scale = 75.0;

    let (mut x_min, mut x_max) = (x_offset, x_offset);

    for (tickname, xposition) in &lower_ticks {
        let x = xposition.log10() * x_scale + x_offset;
        lower_tick(&mut file, x, y_offset, 20.0, 1.75, tickname);
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
    }

    for xposition in &upper_blind_ticks {
        let x = xposition.log10() * x_scale + x_offset;
        upper_blind_tick(&mut file, x, y_offset, 10.0, 0.5);
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
    }

    for (tickname, xposition) in &upper_ticks {
        let x = xposition.log10() * x_scale + x_offset;
        upper_tick(&mut file, x, y_offset, 20.0, 1.75, tickname);
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
    }

    for xposition in &lower_blind_ticks {
        let x = xposition.log10() * x_scale + x_offset;
        lower_blind_tick(&mut file, x, y_offset, 10.0, 0.5);
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
    }

    println!("min {}, max {}", x_min, x_max);
    svg::line(
        &mut file,
        x_min,
        y_offset,
        x_max,
        y_offset,
        1.75,
        "black",
        &svg::StrokeCapType::Square,
    );

    svg::footer(&mut file);

    Ok(())
}
