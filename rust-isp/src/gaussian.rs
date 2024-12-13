use tracing::instrument;

use crate::image::Convolvable;

// Simple
const _SMALL_KERNEL: [[f32; 3]; 3] = [
    [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
    [2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0],
    [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
];

// Sigma 5
const _KERNEL: [[f32; 5]; 5] = [
    [0.0340212472, 0.0368547775, 0.0383588523, 0.0383588523, 0.0368547775],
    [0.0368547775, 0.0399243049, 0.0415536426, 0.0415536426, 0.0399243049],
    [0.0383588523, 0.0415536426, 0.0432494842, 0.0432494842, 0.0415536426],
    [0.0383588523, 0.0415536426, 0.0432494842, 0.0432494842, 0.0415536426],
    [0.0368547775, 0.0399243049, 0.0415536426, 0.0415536426, 0.0399243049],
];

// Sigma 2
const __KERNEL: [[f32; 5]; 5] = [
    [0.0137389963, 0.0226517748, 0.0290854555, 0.0290854555, 0.0226517748],
    [0.0226517748, 0.0373464674, 0.0479538068, 0.0479538068, 0.0373464674],
    [0.0290854555, 0.0479538068, 0.0615739115, 0.0615739115, 0.0479538068],
    [0.0290854555, 0.0479538068, 0.0615739115, 0.0615739115, 0.0479538068],
    [0.0226517748, 0.0373464674, 0.0479538068, 0.0479538068, 0.0373464674],
];

const KERNEL: [[f32; 5]; 5] = [
    [0.0040105069, 0.0111264568, 0.0185325742, 0.0185325742, 0.0111264568],
    [0.0111264568, 0.0308684278, 0.0514154136, 0.0514154136, 0.0308684278],
    [0.0185325742, 0.0514154136, 0.0856391117, 0.0856391117, 0.0514154136],
    [0.0185325742, 0.0514154136, 0.0856391117, 0.0856391117, 0.0514154136],
    [0.0111264568, 0.0308684278, 0.0514154136, 0.0514154136, 0.0308684278],
];

// Sigma 20
const KERNEL20: [[f32; 9]; 9] = [
    [0.0123199821, 0.0123276841, 0.0123334639, 0.0123373186, 0.0123392474, 0.0123392474, 0.0123373186, 0.0123334639, 0.0123276841],
    [0.0123276841, 0.0123353908, 0.0123411743, 0.0123450318, 0.0123469606, 0.0123469606, 0.0123450318, 0.0123411743, 0.0123353908],
    [0.0123334639, 0.0123411743, 0.0123469606, 0.0123508200, 0.0123527497, 0.0123527497, 0.0123508200, 0.0123469606, 0.0123411743],
    [0.0123373186, 0.0123450318, 0.0123508200, 0.0123546803, 0.0123566110, 0.0123566110, 0.0123546803, 0.0123508200, 0.0123450318],
    [0.0123392474, 0.0123469606, 0.0123527497, 0.0123566110, 0.0123585425, 0.0123585425, 0.0123566110, 0.0123527497, 0.0123469606],
    [0.0123392474, 0.0123469606, 0.0123527497, 0.0123566110, 0.0123585425, 0.0123585425, 0.0123566110, 0.0123527497, 0.0123469606],
    [0.0123373186, 0.0123450318, 0.0123508200, 0.0123546803, 0.0123566110, 0.0123566110, 0.0123546803, 0.0123508200, 0.0123450318],
    [0.0123334639, 0.0123411743, 0.0123469606, 0.0123508200, 0.0123527497, 0.0123527497, 0.0123508200, 0.0123469606, 0.0123411743],
    [0.0123276841, 0.0123353908, 0.0123411743, 0.0123450318, 0.0123469606, 0.0123469606, 0.0123450318, 0.0123411743, 0.0123353908],
];

const KERNEL2: [[f32; 11]; 11] = [
    [0.0082529904, 0.0082558561, 0.0082581500, 0.0082598710, 0.0082610184, 0.0082615921, 0.0082615921, 0.0082610184, 0.0082598710, 0.0082581500, 0.0082558561],
    [0.0082558561, 0.0082587237, 0.0082610184, 0.0082627395, 0.0082638869, 0.0082644606, 0.0082644606, 0.0082638869, 0.0082627395, 0.0082610184, 0.0082587237],
    [0.0082581500, 0.0082610184, 0.0082633132, 0.0082650343, 0.0082661826, 0.0082667572, 0.0082667572, 0.0082661826, 0.0082650343, 0.0082633132, 0.0082610184],
    [0.0082598710, 0.0082627395, 0.0082650343, 0.0082667572, 0.0082679056, 0.0082684793, 0.0082684793, 0.0082679056, 0.0082667572, 0.0082650343, 0.0082627395],
    [0.0082610184, 0.0082638869, 0.0082661826, 0.0082679056, 0.0082690539, 0.0082696276, 0.0082696276, 0.0082690539, 0.0082679056, 0.0082661826, 0.0082638869],
    [0.0082615921, 0.0082644606, 0.0082667572, 0.0082684793, 0.0082696276, 0.0082702022, 0.0082702022, 0.0082696276, 0.0082684793, 0.0082667572, 0.0082644606],
    [0.0082615921, 0.0082644606, 0.0082667572, 0.0082684793, 0.0082696276, 0.0082702022, 0.0082702022, 0.0082696276, 0.0082684793, 0.0082667572, 0.0082644606],
    [0.0082610184, 0.0082638869, 0.0082661826, 0.0082679056, 0.0082690539, 0.0082696276, 0.0082696276, 0.0082690539, 0.0082679056, 0.0082661826, 0.0082638869],
    [0.0082598710, 0.0082627395, 0.0082650343, 0.0082667572, 0.0082679056, 0.0082684793, 0.0082684793, 0.0082679056, 0.0082667572, 0.0082650343, 0.0082627395],
    [0.0082581500, 0.0082610184, 0.0082633132, 0.0082650343, 0.0082661826, 0.0082667572, 0.0082667572, 0.0082661826, 0.0082650343, 0.0082633132, 0.0082610184],
    [0.0082558561, 0.0082587237, 0.0082610184, 0.0082627395, 0.0082638869, 0.0082644606, 0.0082644606, 0.0082638869, 0.0082627395, 0.0082610184, 0.0082587237],
];

#[instrument(skip(image))]
pub fn blur<T: Convolvable + Clone>(image: &T) -> T {
    let mut image = image.clone();
    blur_mut(&mut image);
    image
}

#[instrument(skip(image))]
pub fn blur_mut<T: Convolvable + Clone>(image: &mut T) {
    image.convolve_mut::<5, 5>(&KERNEL)
}
