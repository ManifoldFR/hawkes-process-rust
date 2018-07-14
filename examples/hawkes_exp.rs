extern crate pointprocesses;
extern crate plotlib;
extern crate serde_json;

use std::fs;

use plotlib::style::{Point, Marker, Line};
use plotlib::view;
use plotlib::page;
//use plotlib::function;
use plotlib::line;
use plotlib::scatter;
use plotlib::scatter::Scatter;

use pointprocesses::event::Event;
use pointprocesses::hawkes_exponential;

fn main() {
    
    let tmax = 20.0;
    let alpha = 0.5;
    let beta = 0.8;
    let lambda0 = 1.0;

    let events: Vec<Event> = hawkes_exponential(tmax, alpha, beta, lambda0);

    println!("{}", serde_json::to_string_pretty(&events).unwrap());
    
    // Kernel function. Only used for plotting.
    let kernel = |t: f64| {
        if t >= 0.0 {
            alpha*(-beta*t).exp()
        } else {
            0.0
        }
    };

    let intensity_func = |events: &[Event], t: f64| {
        let result: f64 = events
            .iter()
            .take_while(|&ev| ev.timestamp() < t)
            .fold(0.0, |acc, ev| {
            acc+kernel(t - ev.timestamp())
        });
        result + lambda0
    };

    let samples = 1000;
    let times = (0..samples).map(|i| {
        i as f64*tmax/samples as f64
    });
    let mut intens_data: Vec<(f64,f64)> = Vec::new();

    for t in times {
        let lam = intensity_func(&events, t);
        //intens_data.push((t-0.0001/samples as f64, lam-alpha));
        intens_data.push((t, lam));
    };

    let intens_plot = line::Line::new(&intens_data)
        .style(line::Style::new()
            .width(1.5)
            .colour("#4C36EB")
        );

    /*
    let intens_plot = function::Function::new(
        |t| intensity_func(&events, t), 0.0, tmax)
        .style(function::Style::new()
            .width(1.5)
            .colour("#4C36EB"));
    */

    let ev_tupl: Vec<(f64,f64)> = events.into_iter()
        .map(|e: Event| {
            (e.timestamp(), e.intensity())
        }).collect();
    
    let sc = Scatter::from_slice(&ev_tupl)
        .style(scatter::Style::new().size(2.0)
            .colour("#E0A536")
            .marker(Marker::Cross)
        );

    let v = view::ContinuousView::new()
        .x_label("Temps t")
        .y_label("Intensité λ(t)")
        .add(&intens_plot)
        .add(&sc);

    fs::create_dir("examples/images").unwrap_or_default();
    page::Page::single(&v).save("examples/images/hawkes_exp.svg");
}
