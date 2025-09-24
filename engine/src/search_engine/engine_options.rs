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
            ["Contempt"]  contempt:   i64  =>  1000,  -10000,  10000;
            ["DrawScore"] draw_score: i64  =>  25,    -100,    100;
        }
        Tunables {
            //PST
            base_pst:              f64  =>  0.1,   0.01,  1.0,   0.01,   0.002;
            root_pst:              f64  =>  0.34,  0.01,  1.0,   0.034,  0.002;
            depth_pst_adjustment:  f64  =>  1.8,   0.01,  10.0,  0.18,   0.002;
            winning_pst_threshold: f64  =>  0.6,   0.01,  1.0,   0.06,   0.002;
            winning_pst_max:       f64  =>  1.6,   0.01,  10.0,  0.016,  0.002;

            //Node Selection
            start_cpuct:           f64  =>  1.2813,    0.1,    5.0,      0.128,    0.002;
            end_cpuct:             f64  =>  0.3265,    0.0,    1.0,      0.032,    0.002;
            cpuct_depth_decay:     f64  =>  0.264101,  0.0,    5.0,      0.02641,  0.002;
            cpuct_visit_scale:     f64  =>  8000.00,   128.0,  65536.0,  800.0,    0.002;
            cpuct_variance_scale:  f64  =>  0.2,       0.1,    50.0,     0.02,     0.002;
            cpuct_variance_weight: f64  =>  0.85,      0.0,    2.0,      0.085,    0.002;
            cpuct_var_warmup:      f64  =>  0.5,       0.0,    1.0,      0.05,     0.002;
            exploration_tau:       f64  =>  0.51,      0.0,    1.0,      0.055,    0.002;

            //Time manager
            default_moves_to_go:    f64  =>  30.0,         10.0,  50.0,  3.0,      0.002;
            phase_power:            f64  =>  2.0,          0.0,   10.0,  0.2,      0.002;
            phase_scale:            f64  =>  1.0,          0.0,   1.0,   0.1,      0.002;
            soft_constant:          f64  =>  0.0048,       0.0,   1.0,   0.0005,   0.002;
            soft_constant_multi:    f64  =>  0.00032,      0.0,   1.0,   0.00003,  0.002;
            soft_constant_cap:      f64  =>  0.006,        0.0,   1.0,   0.0006,   0.002;
            soft_scale:             f64  =>  0.0125,       0.0,   1.0,   0.0012,   0.002;
            soft_scale_offset:      f64  =>  2.5,          0.0,   10.0,  0.25,     0.002;
            soft_scale_cap:         f64  =>  0.25,         0.0,   1.0,   0.025,    0.002;
            hard_constant:          f64  =>  3.39,         0.0,   10.0,  0.339,    0.002;
            hard_constant_multi:    f64  =>  3.01,         0.0,   10.0,  0.301,    0.002;
            hard_constant_cap:      f64  =>  2.93,         0.0,   10.0,  0.293,    0.002;
            hard_ply_div:           f64  =>  12.0,         0.0,   50.0,  1.2,      0.002;
            hard_scale_cap:         f64  =>  4.0,          0.0,   10.0,  0.4,      0.002;
            bonus_scale:            f64  =>  0.5,          0.0,   1.0,   0.05,     0.002;
            bonus_move_factor:      f64  =>  10.0,         0.0,   50.0,  1.0,      0.002;
            bonus_ply_div:          f64  =>  6.0,          0.0,   20.0,  0.6,      0.002;
            bonus_power:            f64  =>  1.2,          0.0,   10.0,  0.12,     0.002;
            time_fraction:          f64  =>  0.85,         0.0,   1.0,   0.085,    0.002;
            visit_distr_threshold:  f64  =>  0.677245,     0.0,   1.0,   0.07,     0.002;
            visit_penalty_scale:    f64  =>  0.671748,     0.0,   2.0,   0.07,     0.002;
            visit_penalty_multi:    f64  =>  12.014090,    1.0,   50.0,  1.2,      0.002;
            visit_reward_scale:     f64  =>  0.846959,     0.0,   2.0,   0.1,      0.002;
            visit_reward_multi:     f64  =>  11.763412,    1.0,   50.0,  1.2,      0.002;
            gap_threshold:          f64  =>  0.445921,     0.0,   1.0,   0.045,    0.002;
            gap_penalty_scale:      f64  =>  0.227990,     0.0,   2.0,   0.02,     0.002;
            gap_penalty_multi:      f64  =>  18.823099,    1.0,   50.0,  1.8,      0.002;
            gap_reward_scale:       f64  =>  0.132607,     0.0,   2.0,   0.013,    0.002;
            gap_reward_multi:       f64  =>  14.032407,    1.0,   50.0,  1.4,      0.002;
            falling_eval_ema_alpha: f64  =>  0.558354,     0.0,   1.0,   0.055,    0.002;
            falling_eval_multi:     f64  =>  4.658633,     0.0,   10.0,  4.65,     0.002;
            falling_eval_power:     f64  =>  1.898156,     1.0,   3.0,   0.189,    0.002;
            falling_reward_clamp:   f64  =>  0.309756,     0.0,   1.0,   0.03,     0.002;
            falling_penalty_clamp:  f64  =>  0.665333,     0.0,   1.0,   0.066,    0.002;
            instability_ema_alpha:  f64  =>  0.221846,     0.0,   1.0,   0.022,    0.002;
            instability_multi:      f64  =>  0.278716679,  0.0,   1.0,   0.028,    0.002;
            instability_scale:      f64  =>  0.683337,     0.0,   2.0,   0.07,     0.002;
            behind_multi:           f64  =>  0.3882863,    0.0,   1.0,   0.038,    0.002;
            behind_scale:           f64  =>  0.470591,     0.0,   2.0,   0.047,    0.002;
          
            //Transposition Table
            hash_size: f64  =>  0.04,  0.01,  0.5,  0.004,  0.002;

            //Contempt
            max_reasonable_s: f64  =>  2.0,     0.0,    100.0,    0.2,    0.002;
            book_exit_bias:   f64  =>  0.65,    0.0,    1.0,      0.065,  0.002;
            draw_rate_target: f64  =>  0.0,     0.0,    1.0,      0.01,   0.002;
            draw_rate_ref:    f64  =>  0.65,    0.0,    1.0,      0.065,  0.002;
            contempt_att:     f64  =>  1.0,     -10.0,  10.0,     0.1,    0.002;
        }
    }
}