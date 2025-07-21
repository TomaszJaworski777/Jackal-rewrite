use crate::create_options;

mod macros;

create_options! {
    EngineOptions {
        Options {
            ["Hash"] hash: i64  =>  256,  1,  524288;
        }
        Tunables {
            some_tunable: f64  =>  1.01,  0.1,  5.0,  0.055,  0.002;
        }
    }
}