use std::{fs::File, io::Write};

#[allow(dead_code)]
pub enum StrokeCapType {
    Butt,
    Round,
    Square,
}

fn stroke_cap<'a>(stroke_type: &StrokeCapType) -> &'a str {
    match stroke_type {
        StrokeCapType::Square => "stroke-linecap=\"square\"",
        StrokeCapType::Round => "stroke-linecap=\"round\"",
        StrokeCapType::Butt => "stroke-linecap=\"butt\"",
    }
}

#[allow(dead_code)]
pub enum StrokeJoinType {
    Miter,
    Round,
    // Miter,
}

#[allow(dead_code)]
fn stroke_joint(stroke_type: &StrokeJoinType) -> String {
    match &stroke_type {
        StrokeJoinType::Miter => "stroke-linejoin=\"miter\"".to_string(),
        StrokeJoinType::Round => "stroke-linejoin=\"round\"".to_string(),
    }
}

// #[allow(dead_code)]
// pub struct SVGFile<'a> {
//     canvas_size: (f64, f64),
//     text_styles: Vec<String>,
//     stroke_joint_style: StrokeStyle,
//     stroke_cap_style: StrokeStyle,
//     file: &'a mut File,
// }

// trait SVGFileObject {
//     fn new(filename: &'static str, width: f64, height: f64) -> Self;

// }

// impl SVGFile<'static> {
//     fn prepare_file(&mut self, width: f64, height: f64) {
//         header(&mut self.file, width, height);
//     }
// }

/// setup
/// init: open file,
/// set style: text style 1, text style 2, ...
/// set joint style
/// set cap style
/// write header
/// write vector objects
/// write footer
/// close file (desctructor?)

pub fn header(file: &mut File, width: f64, height: f64) {
    let mut string = String::new();
    string.push_str("<?xml version=\"1.0\" encoding=\"utf-8\" ?>");
    string.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:ev=\"http://www.w3.org/2001/xml-events\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" ");
    string.push_str("baseProfile=\"tiny\" version=\"1.2\" ");
    string.push_str(&format!(
        "width=\"100%\" height=\"100%\" viewBox=\"{},{},{},{}\">",
        0.0, 0.0, width, height
    ));
    string.push_str("<defs />");
    write!(file, "{}", string).expect("couldn't write file");
}

pub fn footer(file: &mut File) {
    write!(file, "</svg>").expect("couldn't write file");
}

pub fn line(
    file: &mut File,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    thickness: f64,
    color: &'static str,
    cap_type: &StrokeCapType,
) {
    // println!("{}", stroke_cap(&cap_type));
    write!(
        file,
        "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" {} stroke=\"{}\" stroke-width=\"{}\" />",
        x1,
        y1,
        x2,
        y2,
        stroke_cap(&cap_type),
        color,
        thickness
    )
    .expect("couldn't write file");
}

pub fn text_start_aligned(file: &mut File, text: &str, x: f64, y: f64, rotate: i16) {
    write!(
        file,
        "<text text-anchor=\"start\" transform=\"translate({0}, {1}) rotate({2})\">{3}</text>",
        x, y, rotate, text
    )
    .expect("could not write file!");
}
#[allow(dead_code)]
pub fn text_end_aligned(file: &mut File, text: &str, x: f64, y: f64, rotate: i16) {
    write!(
        file,
        "<text text-anchor=\"end\" transform=\"translate({0}, {1}) rotate({2})\">{3}</text>",
        x, y, rotate, text
    )
    .expect("could not write file!");
}

pub fn set_text_style(file: &mut File, fontsize: u16) {
    let mut s = String::new();

    s.push_str(&format!(
        "<style>\n.style_1 {{ font: italic {}px sans-serif; }}\n</style>",
        fontsize
    ));
    // set text style for class 'small'
    // s.push_str("<style>\n.small { font: italic 13px sans-serif; }\n.heavy { font: bold 30px sans-serif; } </style>");
    write!(file, "{}", s).expect("could not write file.");
    /* Note that the color of the text is set with the    *
     * fill property, the color property is for HTML only */
    // .Rrrrr { font: italic 40px serif; fill: red; }
}

// pub fn circle() {};
// pub fn rectangle() {};
// pub fn circle() {};
// pub fn circle() {};
// pub fn text() {};
