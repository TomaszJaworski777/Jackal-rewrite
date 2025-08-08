use crate::create_options;

mod macros;

create_options! {
    EngineOptions {
        Options {
            //====== General ======
            ["Hash"]         hash:          i64   =>  256,    1,  524288;
            ["Threads"]      threads:       i64   =>  1,      1,  1024;
            ["MoveOverhead"] move_overhead: i64   =>  100,    0,  2000;
            ["MultiPV"]      multi_pv:      i64   =>  1,      1,  218;
            ["UCI_Chess960"] chess960:      bool  =>  false;
            ["UCI_ShowWDL"]  show_wdl:      bool  =>  false;
            ["Report_iters"] report_iters:  bool  =>  false;

            //======== EAS ========

        }
        Tunables {
            //PST
            root_pst:   f64  =>  3.25,  0.1,  10.0,  0.4,  0.002;
            common_pst: f64  =>  1.00,  0.1,  10.0,  0.1,  0.002;

            //Node Selection
            cpuct:                 f64  =>  0.75,     0.1,    5.0,      0.075,  0.002;
            cpuct_visit_scale:     f64  =>  8000.00,  128.0,  65536.0,  800.0,  0.002;
            cpuct_variance_scale:  f64  =>  0.2,      0.1,    50.0,     0.02,   0.002;
            cpuct_variance_weight: f64  =>  0.85,     0.0,    2.0,      0.085,  0.002;
            exploration_scale:     f64  =>  0.55,     0.0,    1.0,      0.05,   0.002;
            gini_base:             f64  =>  0.679,    0.0,    2.0,      0.085,  0.002;
            gini_scale:            f64  =>  1.634,    0.0,    2.0,      0.085,  0.002;
            gini_min:              f64  =>  2.1,      0.0,    2.0,      0.085,  0.002;

            //Transposition Table
            hash_size: f64  =>  0.04,  0.01,  0.5,  0.004,  0.002;
        }
    }
}