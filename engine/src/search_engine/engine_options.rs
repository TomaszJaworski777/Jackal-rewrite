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
            base_pst:               f64  =>  0.1,   0.01,  1.0,   0.01,   0.002;
            root_pst:               f64  =>  0.34,  0.01,  1.0,   0.034,  0.002;
            depth_pst_adjustment:   f64  =>  1.8,   0.01,  10.0,  0.18,   0.002;
            winning_pst_threshold:  f64  =>  0.6,   0.01,  1.0,   0.06,   0.002;
            winning_pst_max:        f64  =>  1.6,   0.01,  10.0,  0.016,  0.002;

            //Node Selection
            cpuct:                 f64  =>  1.15,     0.1,    5.0,      0.075,  0.002;
            cpuct_visit_scale:     f64  =>  8000.00,  128.0,  65536.0,  800.0,  0.002;
            cpuct_variance_scale:  f64  =>  0.2,      0.1,    50.0,     0.02,   0.002;
            cpuct_variance_weight: f64  =>  0.85,     0.0,    2.0,      0.085,  0.002;
            cpuct_depth_scale:     f64  =>  4.7845,   1.0,    10.0,     0.478,  0.002;
            cpuct_min_depth_mul:   f64  =>  0.3517,   0.0,    1.0,      0.035,  0.002;
            exploration_scale:     f64  =>  0.53,     0.0,    1.0,      0.055,  0.002;

            //Transposition Table
            hash_size: f64  =>  0.04,  0.01,  0.5,  0.004,  0.002;
        }
    }
}