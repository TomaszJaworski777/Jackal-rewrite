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
            ["UCI_ShowWDL"]  show_wdl:      bool  =>  false;
            ["Report_iters"] report_iters:  bool  =>  false;

            //======== EAS ========

        }
        Tunables {
            root_pst:   f64  =>  3.25,  0.1,   10.0,  0.4,    0.002;
            common_pst: f64  =>  1.00,  0.1,   10.0,  0.1,    0.002;
            hash_size:  f64  =>  0.04,  0.01,  0.5,   0.004,  0.002;
        }
    }
}