use std::path::Path;
use std::fs::File;
use std::env;

use std::io::Write;

const MAX_DIMS: usize = 4;
const MAX_ORDER: usize = 6;

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&out).join("macro_invocs.rs")).unwrap();

    let dim_tags = ['x', 'y', 'z', 'w'];

    // Create points and vectors
    for dim in 2..(MAX_DIMS + 1) {
        writeln!(file, "n_pointvector!{{{0}; Point{0}d, Vector{0}d {{", dim).unwrap();

        for (i, dt) in dim_tags[0..dim].iter().enumerate() {
            write!(file, "    {}", dt).unwrap();

            if i == dim - 1 {
                writeln!(file, "")
            } else {
                writeln!(file, ",")
            }.unwrap();
        }

        writeln!(file, "}}}}").unwrap();
    }

    // Create one-dimensional bezier polynomials
    for order in 2..(MAX_ORDER + 1) {
        writeln!(file, "n_bezier!{{BezPoly{}o {{", order).unwrap();
        for o in 0..(order + 1) {
            write!(file, "    {}: {}", get_param_name(o, order), combination(order, o)).unwrap();

            // Insert commas necessary to seperate parameter names. 
            if o != order {
                writeln!(file, ",")
            } else {
                writeln!(file, "")
            }.unwrap();
        }


        // Order of the derivative of the polynomial
        let dorder = order - 1;
        
        writeln!(file, "}} derived {{").unwrap();
        for o in 0..order {
            write!(file, "    {} - {}: {}", get_param_name(o+1, order), get_param_name(o, order), combination(dorder, o)).unwrap();
            
            if o != dorder {
                writeln!(file, ",")
            } else {
                writeln!(file, "")
            }.unwrap();
        }
        writeln!(file, "}}}}").unwrap();
    }

    // Create composite curves
    for dim in 2..(MAX_DIMS + 1) {
        for order in 2..(MAX_ORDER + 1) {        
            writeln!(file, "bez_composite!{{Bez{0}o{1}d<BezPoly{0}o> {{", order, dim).unwrap();

            for dt in &dim_tags[0..dim] {
                writeln!(file, "    {}:", dt).unwrap();

                for o in 0..(order + 1) {
                    write!(file, "        {}_{}", dt, get_param_name(o, order)).unwrap();

                    if o == order {
                        writeln!(file, ";")
                    } else {
                        writeln!(file, ",")
                    }.unwrap();
                }
            }
            writeln!(file, "}} -> <Point{0}d; Vector{0}d>;", dim).unwrap();

            for o in 0..(order + 1) {
                write!(file, "    {}:", get_param_name(o, order)).unwrap();

                for (i, dt) in dim_tags[0..dim].iter().enumerate() {
                    write!(file, " {}", dt).unwrap();

                    if i != dim-1 {
                        write!(file, ",").unwrap();
                    }
                }
                writeln!(file, ";").unwrap();
            }

            writeln!(file, "}}").unwrap();
        }
    }
}

fn get_param_name(param_number: usize, poly_order: usize) -> String {
    if 0 == param_number {
        "start".to_owned()
    } else if param_number == poly_order {
        "end".to_owned()
    } else if 2 == poly_order {
        "ctrl".to_owned()
    } else {
        format!("ctrl{}", param_number - 1)
    }
}

fn combination(n: usize, k: usize) -> usize {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn factorial(n: usize) -> usize {
    match n {
        0 => 1,
        _ => n * factorial(n-1)
    }
}