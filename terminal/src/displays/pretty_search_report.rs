use std::io::{self, Write};

use engine::{PvLine, SearchEngine, SearchLimits, SearchReport, SearchStats, Tree};
use utils::{bytes_to_string, clear_terminal_screen, create_loading_bar, heat_color, number_to_string, time_to_string, AlignString, Theme, DRAW_COLOR, LOSE_COLOR, WIN_COLOR};

static mut SEARCH_HISTORY: Vec<(u128, PvLine)> = Vec::new();

pub struct PrettySearchReport;
impl SearchReport for PrettySearchReport {
    fn refresh_rate_per_second() -> f64 {
        20.0
    }

    fn search_started(_: &SearchLimits, _: &SearchEngine) {
        clear_terminal_screen();
        
        unsafe {
            #[allow(static_mut_refs)]
            SEARCH_HISTORY.clear();
        }

        print!("\x1B[?25l");
        let _ = io::stdout().flush();
    }

    fn search_report(search_limits: &SearchLimits, search_stats: &SearchStats, search_engine: &SearchEngine) { 
        print_search_report::<false>(search_limits, search_stats, search_engine);
    }

    fn search_ended(search_limits: &SearchLimits, search_stats: &SearchStats, search_engine: &SearchEngine) {
        print_search_report::<true>(search_limits, search_stats, search_engine);

        let best_child_idx = search_engine
                .tree()
                .select_child_by_key(0, |child| child.score().single(0.5) as f64);

        if let Some((x,y)) = term_cursor::get_pos().ok() {
            let _ = term_cursor::set_pos(x, y - 2);
        }

        print!("{}\r", " ".repeat(50));
        println!( "\n{}",
            format!(" Best Move: {}", search_engine
                .tree()
                .get_child_copy(search_engine.tree().root_index(), best_child_idx.unwrap())
                .mv()
                .to_string(search_engine.options().chess960()).secondary(1.0)).primary(1.0)
        );

        print!("\x1B[?25h");
        let _ = io::stdout().flush();
    } 
}

const PV_WRAPPING: usize = 13;

fn print_search_report<const FINAL: bool>(_: &SearchLimits, search_stats: &SearchStats, search_engine: &SearchEngine) {
    let grad = |a: u8| -> f32 {
        a as f32 / 33.0
    };

    let (t_width, t_height) = if let Some(dims) = term_size::dimensions() {
        dims
    } else {
        (80, 25)
    };

    #[cfg(target_os = "linux")]
    let t_height = t_height - 1;

    let mut height_used = 0;

    let _ = term_cursor::set_pos(0, 0);

    if t_height >= 36 {
        for _ in 0..11  {
            println!("{}", " ".repeat(t_width));
        }

        let _ = term_cursor::set_pos(0, 0);

        search_engine.current_position().board().draw_board();
        height_used += 11;
    }

    if t_height >= 25 {
        let tree_size_nodes = search_engine.tree().tree_size();
        let tree_bytes = bytes_to_string(Tree::size_to_bytes(tree_size_nodes) as u128);
        let tree_size = number_to_string(tree_size_nodes as u128);

        let current_size = search_engine.tree().current_index().min(tree_size_nodes);
        let usage = current_size as f32 / tree_size_nodes as f32;

        print!("{}\r", " ".repeat(t_width));
        println!("{}", format!(" Threads:    {}", search_engine.options().threads().to_string().secondary(grad(11))).primary(grad(11)));
        print!("{}\r", " ".repeat(t_width));
        println!("{}", format!(" Tree Size:  {} | {}", format!("{}n", tree_size).secondary(grad(12)), format!("{}B", tree_bytes).secondary(grad(12))).primary(grad(12)));
        print!("{}\r", " ".repeat(t_width));
        println!("{}", format!(" Tree Usage: {}", create_loading_bar(50, usage, WIN_COLOR, LOSE_COLOR).secondary(grad(13))).primary(grad(13)));

        print!("{}\r", " ".repeat(t_width));
        println!();

        height_used += 4;
    }

    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Avg. Depth: {}", search_stats.avg_depth().to_string().secondary(grad(15))).primary(grad(15)));
    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Max Depth:  {}", search_stats.max_depth().to_string().secondary(grad(16))).primary(grad(16)));

    print!("{}\r", " ".repeat(t_width));
    println!();

    height_used += 3;

    if t_height >= 21 {
        let nodes = if search_engine.options().report_iters() {
            search_stats.iterations()
        } else {
            search_stats.cumulative_depth()
        };

        let time = search_stats.time_passesd_ms();

        let nps = (nodes as u128 * 1000) / time.max(1);

        print!("{}\r", " ".repeat(t_width));
        println!("{}", format!(" Nodes:      {}", number_to_string(nodes as u128).secondary(grad(18))).primary(grad(18)));
        print!("{}\r", " ".repeat(t_width));
        println!("{}", format!(" Time:       {}", time_to_string(time).secondary(grad(19))).primary(grad(19)));
        print!("{}\r", " ".repeat(t_width));
        println!("{}", format!(" Nps:        {}", number_to_string(nps).secondary(grad(20))).primary(grad(20)));

        print!("{}\r", " ".repeat(t_width));
        println!();

        height_used += 4;
    }

    let pv = search_engine.tree().get_best_pv(0);
    let single = pv.score().single(0.5);
    let score = match pv.state() {
        engine::GameState::Loss(len) => format!("+M{}", (len + 1).div_ceil(2)),
        engine::GameState::Win(len) => format!("-M{}", (len + 1).div_ceil(2)),
        _ => format!("{}{:.2}", if single < 0.5 { "-" } else { "+" }, pv.score().cp(0.5).abs() as f32 / 100.0)
    };

    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Score:      {}", heat_color(score.as_str(), single as f32, 0.0, 1.0)).primary(grad(22)));
    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Win:        {}", create_loading_bar(50, pv.score().win_chance() as f32, WIN_COLOR, WIN_COLOR).secondary(grad(23))).primary(grad(23)));
    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Draw:       {}", create_loading_bar(50, pv.score().draw_chance() as f32, DRAW_COLOR, DRAW_COLOR).secondary(grad(24))).primary(grad(24)));
    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Lose:       {}", create_loading_bar(50, pv.score().lose_chance() as f32, LOSE_COLOR, LOSE_COLOR).secondary(grad(25))).primary(grad(25)));

    unsafe {
        #[allow(static_mut_refs)]
        if SEARCH_HISTORY.len() == 0 || SEARCH_HISTORY.last().unwrap().1.first_move() != pv.first_move() {
            SEARCH_HISTORY.push((search_stats.time_passesd_ms(), pv.clone()));
        }
    }

    let pv_string = if FINAL {
        pv.to_string(search_engine.options().chess960())
    } else {
        pv.to_string_wrapped(PV_WRAPPING, search_engine.options().chess960())
    };

    height_used += pv_string.len().div_ceil(t_width - 13);

    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Best Line:  {}", pv_string.secondary(grad(26))).primary(grad(26)));
    print!("{}\r", " ".repeat(t_width));
    println!();

    height_used += 5;

    print!("{}\r", " ".repeat(t_width));
    println!("{}", format!(" Search History:").primary(grad(28)));

    #[allow(static_mut_refs)]
    unsafe {
        let start_idx = (SEARCH_HISTORY.len() as i32 - (t_height - height_used - 4) as i32).max(0) as usize;
        for idx in start_idx..SEARCH_HISTORY.len() {
            let (time, pv) = &SEARCH_HISTORY[idx];

            let pv_string = pv.to_string_wrapped(PV_WRAPPING, search_engine.options().chess960());

            print!("{}\r", " ".repeat(t_width));
            println!("{}", format!("{} -> {}", time_to_string(*time).align_to_right(9), pv_string).secondary(grad(29)))
        }
    }

    print!("{}\r", " ".repeat(t_width));
    println!();
    print!("{}\r", " ".repeat(t_width));
    println!();
}