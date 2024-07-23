mod axis;
mod hist;

use crate::axis::Fill;

fn main() {
    // uniform axis
    // let uniform_ax = axis::Axis::from_uniform(
    //     String::from("uniform axis"),
    //     0.0, // start
    //     3.0, // stop
    //     5, // nbins
    // );

    // variable axis
    let variable_ax = axis::Axis::from_edges(
        String::from("variable axis"),
        vec![0.0, 1.1, 2.4, 3.0],
    );

    // manual axis building
    // let manual_ax = axis::Axis::new(
    //     String::from("manual axis"),
    //     vec![
    //         axis::Bin::new(0.0, 1.1),
    //         axis::Bin::new(1.1, 2.4),
    //         axis::Bin::new(2.4, 3.0),
    //     ],
    // );

    let mut histogram = hist::Histogram::new(variable_ax);

    let _ = histogram.weighted_fill(1.5, 1.2);
    let _ = histogram.fill(0.7);

    println!("{:#?}", histogram);
}
