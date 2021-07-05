use nalgebra::base::DMatrix;
use nalgebra::base::DVector;
use nalgebra::Complex;
use rand::distributions::Standard;
use rand::prelude::*;
use rand_pcg::Pcg64;

use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "out/dist.png";
const RANDOM_SEED: u64 = 42;

fn random_matrix(r: usize, c: usize) -> DMatrix<f64> {
    let rng = Pcg64::seed_from_u64(RANDOM_SEED);
    let mut mat = DMatrix::from_iterator(r, c, rng.sample_iter(Standard).take(r * c));
    // Scale 0.0..1.0 to -sqrt(12)/2..sqrt(12)/2 to make mean 0.0, variance 1.0.
    mat.add_scalar_mut(-0.5);
    mat.scale_mut(12f64.sqrt());
    mat
}

// Based on https://github.com/38/plotters/blob/master/examples/normal-dist.rs
fn plot_complex(v: &DVector<Complex<f64>>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 1024)).into_drawing_area();

    root.fill(&WHITE)?;

    let random_points: Vec<(f64, f64)> = v
        .iter()
        .map(|c| (c.re, c.im))
        .collect();

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-1f64..1f64, -1f64..1f64)?;

    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    scatter_ctx.draw_series(
        random_points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'out' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 1000;
    let mat = random_matrix(n, n);
    let mean = mat.iter().sum::<f64>() / mat.len() as f64;
    let var = mat.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / mat.len() as f64;
    if cfg!(debug) {
        println!("mean: {} var: {}", mean, var);
    }
    let mut eigenvalues = mat.complex_eigenvalues();
    // Normalise to unit circle.
    eigenvalues.unscale_mut((n as f64).sqrt());
    if cfg!(debug) {
        println!("{:}", &eigenvalues);
    }
    plot_complex(&eigenvalues)?;
    Ok(())
}
