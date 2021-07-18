use nalgebra::base::DMatrix;
use nalgebra::base::DVector;
use nalgebra::Complex;
use rand::distributions::Standard;
use rand::prelude::*;
use rand_pcg::Pcg64;

use plotters::prelude::*;

const OUT_FILE_NAME: &str = "out/dist.gif";
const FRAME_DELAY: u32 = 10;

// After a bit of trial and error, I like this one.
const RANDOM_SEED: u64 = 116;

const MATRIX_SIZE: usize = 100;
const STEPS: usize = 50;

////////////////////////////////////////////////////////////////////////
// Random matrix generation
//

fn random_matrix(r: usize, c: usize) -> DMatrix<f64> {
    let rng = Pcg64::seed_from_u64(RANDOM_SEED);
    let mut mat = DMatrix::from_iterator(r, c, rng.sample_iter(Standard).take(r * c));
    // Scale 0.0..1.0 to -sqrt(12)/2..sqrt(12)/2 to make mean 0.0, variance 1.0.
    mat.add_scalar_mut(-0.5);
    mat.scale_mut(12f64.sqrt());
    mat
}

////////////////////////////////////////////////////////////////////////
// Graph plotting functions
//
// Based on https://github.com/38/plotters/blob/master/examples/normal-dist.rs
//

// Generate a drawing area we will use over multiple frames.
//
// The type paramter to BitMapBackend is... the lifetime of the file
// name. Which apparently pollutes everything (see plot_complex). *sigh*
fn new_plot(
) -> Result<DrawingArea<BitMapBackend<'static>, plotters::coord::Shift>, Box<dyn std::error::Error>>
{
    let backend = BitMapBackend::gif(OUT_FILE_NAME, (1024, 1024), FRAME_DELAY)?;
    Ok(backend.into_drawing_area())
}

// Plot a set of points into the given drawing area.
fn plot_complex<DB: DrawingBackend>(
    drawing_area: &mut DrawingArea<DB, plotters::coord::Shift>,
    v: &DVector<Complex<f64>>,
    highlighted: Complex<f64>,
) -> Result<(), Box<dyn std::error::Error>>
where
    <DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
    drawing_area.fill(&WHITE)?;

    let points: Vec<(f64, f64)> = v.iter().map(|c| (c.re, c.im)).collect();

    let mut scatter_ctx = ChartBuilder::on(drawing_area)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-1f64..1f64, -1f64..1f64)?;

    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    scatter_ctx.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, BLACK.filled())),
    )?;

    scatter_ctx.draw_series([
        Circle::new((highlighted.re, highlighted.im), 5, GREEN.filled()),
        // And the complex conjugate, knowing the eigenvalues of a real
        // matrix come in pairs.
        Circle::new((highlighted.re, -highlighted.im), 5, GREEN.filled()),
    ])?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    drawing_area.present().expect(
        "Unable to write result to file, please make sure 'out' dir exists under current dir",
    );
    if cfg!(debug) {
        println!("Frame has been written to {}", OUT_FILE_NAME);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////
// And the core animation...
//

// Linearly interpolate an n x n from having the nth dimension being a
// diagonal element only (i.e. completely orthogonal from all other
// dimensions), to the full random maatrix.
//
// A "lerp" of 0 has an orthogonal eigenvector, 1 is the full matrix.
//
// "highlighted" is the last-seen location of the newest eigenvalue,
// so we can highlight its movement. We return an updated loation for the
// next iteration.

fn plot_lerp_matrix<DB: DrawingBackend>(
    drawing_area: &mut DrawingArea<DB, plotters::coord::Shift>,
    base_mat: &DMatrix<f64>,
    lerp: f64,
    highlighted: Complex<f64>,
) -> Result<Complex<f64>, Box<dyn std::error::Error>>
where
    <DB as plotters::prelude::DrawingBackend>::ErrorType: 'static,
{
    let mut mat = base_mat.clone();

    assert_eq!(mat.nrows(), mat.ncols());
    // Lerp the last row and column, except for the bottom-right element.
    let n = mat.nrows() - 1;
    // NB: Upper limit skips last element.
    for i in 0..n {
        mat[(i, n)] *= lerp;
        mat[(n, i)] *= lerp;
    }

    if cfg!(debug) {
        let mean = mat.iter().sum::<f64>() / mat.len() as f64;
        let var = mat.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / mat.len() as f64;
        println!("mean: {} var: {}", mean, var);
    }

    // Extract eigenvalues and normalise to unit circle.
    let mut eigenvalues = mat.complex_eigenvalues();
    eigenvalues.unscale_mut((mat.nrows() as f64).sqrt());

    if cfg!(debug) {
        println!("{}", &eigenvalues);
    }

    // Update 'highlighted' to point to the nearest eigenvalue.
    let highlighted_new = *eigenvalues
        .iter()
        .min_by(|&a, &b| {
            let an = (a - highlighted).norm_sqr();
            let bn = (b - highlighted).norm_sqr();
            an.partial_cmp(&bn).unwrap()
        })
        .unwrap();

    plot_complex(drawing_area, &eigenvalues, highlighted_new)?;
    Ok(highlighted_new)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mat = random_matrix(MATRIX_SIZE, MATRIX_SIZE);
    let mut drawing_area = new_plot()?;

    // The initial eigenvalue is the associated diagonal element.
    let mut highlighted = Complex::new(mat[(MATRIX_SIZE - 1, MATRIX_SIZE - 1)], 0.0);
    // And we need to normalise it, like all displayed points:
    highlighted /= (mat.nrows() as f64).sqrt();

    for lerp_step in 0..STEPS {
        // This could be slow, so let's log progress.
        println!("Generating frame for {} of {}", lerp_step + 1, STEPS);

        let lerp = lerp_step as f64 / (STEPS - 1) as f64;
        highlighted = plot_lerp_matrix(&mut drawing_area, &mat, lerp, highlighted)?;
    }
    Ok(())
}
