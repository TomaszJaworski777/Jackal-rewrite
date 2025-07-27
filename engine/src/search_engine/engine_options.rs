use crate::create_options;

mod macros;

create_options! {
    EngineOptions {
        Options {
            //====== General ======
            ["Hash"]            hash:          i64   =>  256,    1,  524288;
            ["Threads"]         threads:       i64   =>  1,      1,  1024;
            ["MoveOverhead"]    move_overhead: i64   =>  100,    0,  2000;
            ["MultiPV"]         multi_pv:      i64   =>  1,      1,  218;
            ["UCI_ShowWDL"]     show_wdl:      bool  =>  false;
            ["UCI_AnalyseMode"] analyse_mode:  bool  =>  false;

            //======== EAS ========

        }
        Tunables {
            some_tunable: f64  =>  1.01,  0.1,  5.0,  0.055,  0.002;
        }
    }
}