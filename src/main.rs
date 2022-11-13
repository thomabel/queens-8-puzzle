/*
Thomas Abel
AI
2022-10-27
*/
mod tests;
mod vector;
mod gene;
mod genetic_algorithm;
fn main() {
    experiment();
}


use genetic_algorithm::GeneticAlgorithm;
use vector::Vector2;
use plotters::prelude::*;

fn experiment() {
    let board_size = Vector2::new(8, 8);
    let population_size = usize::pow(2, 8);
    let num_iterations = usize::pow(2, 12);
    let mutate_pct = 0.;

    let mut genetic_alg = GeneticAlgorithm::new(board_size, population_size);
    let result = genetic_alg.run(num_iterations, mutate_pct);

    for (i, avg) in result.avg_fitness.iter().enumerate() {
        if i % 1000 == 0 {
            println!("Iteration: {:5} ,  Avg: {:3.3}", i, avg);
        }
    }
    let name = format!("Psize{} iter{} mut{:0.3}", population_size, num_iterations, mutate_pct);
    let caption = format!("Pop_size: {:5}  , num_iterations: {:6}  , mutate_pct: {:0.3}", population_size, num_iterations, mutate_pct);
    println!("{}\n# of Solutions: {}", caption, result.correct.len());

    let _r = visualize(&result, name, caption, num_iterations);
}

// Uses plotters crate to visualize data.
fn visualize(result: &genetic_algorithm::Result, name: String, caption: String, num_iterations: usize,
) -> Result<(), Box<dyn std::error::Error>>
{
    let path = format!("./plots/{}.png", name);
    let size = (1290, 720);
    let root = BitMapBackend::new(&path, size).into_drawing_area();
    let dimension = (0.0..(num_iterations as f32), 0.0..28f32);
    let line_color = BLUE;
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(dimension.0, dimension.1)?;

    chart.configure_mesh().draw()?;
 
     // Plot the train set line.
     let iter
         = result.avg_fitness.iter().enumerate()
         .map(|x| (x.0 as f32, *x.1));

     chart.draw_series(LineSeries::new(iter, line_color.filled()).point_size(2))?
         .label("Fitness")
         .legend(|(x, y)|
            PathElement::new(vec![(x, y), (x + 20, y)], line_color)
        );
 
     // Create the line key.
     chart
     .configure_series_labels()
     .background_style(WHITE.mix(0.8))
     .border_style(BLACK)
     .draw()?;
 
     root.present()?;
     Ok(())
}
