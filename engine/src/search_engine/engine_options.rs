use crate::create_options;

mod macros;

create_options! {
    EngineOptions {
        Options {
            //====== General ======
            ["Hash"]         hash:          i64   =>  32,  1,  524288;
            ["Threads"]      threads:       i64   =>  1,   1,  1024;
            ["MoveOverhead"] move_overhead: i64   =>  25,  0,  2000;
            ["MultiPV"]      multi_pv:      i64   =>  1,   1,  218;
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
            start_cpuct:           f64  =>  1.2813,   0.1,    5.0,      0.128,  0.002;
            end_cpuct:             f64  =>  0.3265,   0.0,    1.0,      0.032,  0.002;
            cpuct_depth_decay:     f64  =>  26.4101,  0.0,    500.0,    2.641,  0.002;
            cpuct_visit_scale:     f64  =>  8000.00,  128.0,  65536.0,  800.0,  0.002;
            cpuct_variance_scale:  f64  =>  0.2,      0.1,    50.0,     0.02,   0.002;
            cpuct_variance_weight: f64  =>  0.85,     0.0,    2.0,      0.085,  0.002;
            exploration_tau:       f64  =>  0.51,     0.0,    1.0,      0.055,  0.002;

            //Transposition Table
            hash_size: f64  =>  0.04,  0.01,  0.5,  0.004,  0.002;
        }
    }
}