use ndarray::prelude::*;
use rust_dmi::dmi::DMI;

pub use rust_dmi::dmi::DMIError;

struct SubstrateDMI;
impl DMI for SubstrateDMI {}

// A Substrate-friendly wrapper around the rust dmi calculation function
pub fn calculate_dmi(
    answers: Vec<usize>,
    dimensions: (usize, usize),
    choice_n: usize
) -> Result<Vec<f32>, DMIError> {
    let answers = Array2::from_shape_vec(dimensions, answers).unwrap();
    SubstrateDMI::calculate_dmi(answers, choice_n)
}
