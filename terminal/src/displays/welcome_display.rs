use utils::{lerp_color, CustomColor, DARK_WHITE, GRAY, ORANGE};

pub fn welcome_message() -> String {
    let mut result = String::default();

    let logo_lerp = |idx: usize| -> (u8, u8, u8) {
        lerp_color(DARK_WHITE, GRAY, idx as f32 / 24.0)
    };

    let logo: [String; 25] = [
        format!("                   {}  .-=========-. {}                   ", "=.".custom_color(logo_lerp(0)), ".=--".custom_color(logo_lerp(0))),
        format!("                 . {}===========:{}.                   ", "--.".custom_color(logo_lerp(1)), ".:+=.-".custom_color(logo_lerp(1))),
        format!("              .:==.{}.==-.               ", "::-..:+=. .. :+=:. -.".custom_color(logo_lerp(2))),
        format!("            .=====-   {}======:             ", "+=...... ==:   -..".custom_color(logo_lerp(3))),
        format!("          .-======{}==-====:.          ", ":.:-=++:-:..--.  .:..:-".custom_color(logo_lerp(4))),
        format!("         :====={}=======.         ", "-..*++##=:::::.   .   .:.  -".custom_color(logo_lerp(5))),
        format!("        -===={}:=====:        ", ". +==:+=-.:::.::::::.    .:-==..".custom_color(logo_lerp(6))),
        format!("       -====={} .=: {}======:       ", "...-...".custom_color(logo_lerp(7)), ".::::::..  ...:::  :-".custom_color(logo_lerp(7))),
        format!("      :======-{}:======-.      ", ".+--.  ...::::::...   ..::::-=.".custom_color(logo_lerp(8))),
        format!("     .---=-:{}-=-----      ", ".++-::::::...::::::.    ...:....-=.".custom_color(logo_lerp(9))),
        format!("     :--:{}-----.     ", ".-+:::-:::::::::::::..:.   ..::::. ..-:.".custom_color(logo_lerp(10))),
        format!("    .: {}      ----:     ", ".-: .::-::::.        ...   .. ..:::::".custom_color(logo_lerp(11))),
        format!("    .:{} -------:     ", ":  .::::::.            .. .  ...:.....:.".custom_color(logo_lerp(12))),
        format!("    .:--{}------:     ", ". ......------:    .    .. .:....... . .".custom_color(logo_lerp(13))),
        format!("    .:----{}------{} :-----.     ", ". :..".custom_color(logo_lerp(14)), ":    ..:.  ...:::.:..    .".custom_color(logo_lerp(14))),
        format!("     .------------:{}----:.     ", "..  . ...::. :::::..:.       .".custom_color(logo_lerp(15))),
        format!("     .::-------:{}:::.      ", ".    ..  .:::..:-::....:.      :..".custom_color(logo_lerp(16))),
        format!("      .::::---:{}::::.       ", ".  ... .:---:.:--::..:....      .".custom_color(logo_lerp(17))),
        format!("       .:::::..:{}:::.       ", " ....:=--:..:-:....:. ..        .".custom_color(logo_lerp(18))),
        format!("        .:::::{}           ::..         ", ". ... :+--:....:.. ...".custom_color(logo_lerp(19))),
        format!("          .::{}::.          ", ".  ....:-::.....   ..        ..  .".custom_color(logo_lerp(20))),
        format!("           ..{}            ", ".   .. .:::...          .   ..  ...".custom_color(logo_lerp(21))),
        format!("                {}              ", ".    ..:.          ..   .   ..".custom_color(logo_lerp(22))),
        format!("              . ...  ....  .    ...   ..                    ").custom_color(logo_lerp(23)),
        format!("                  ..   .. ..  ..     .                      ").custom_color(logo_lerp(24)),
    ];

    let bg_lerp = |idx: usize| -> (u8, u8, u8) {
        let (orange_r, orange_g, orange_b) = ORANGE;
        let r = orange_r as i16 + idx as i16 * 4;
        let g = orange_g as i16 - idx as i16 * 3 + 25;
        let b = orange_b as i16 - idx as i16 * 3 + 25;
        (r.clamp(0, 255) as u8, g.clamp(0, 255) as u8, b.clamp(0, 255) as u8)
    };

    let logo: [String; 25] = {
        let mut result = std::array::from_fn(|_| String::new());

        for (idx, line) in logo.into_iter().enumerate() {
            result[idx] = line.custom_color(bg_lerp(idx))
        }

        result
    };

    let info: [String; 25] = {
        let mut result = std::array::from_fn(|_| String::new());

        result[4] = "Jackal Chess Engine".custom_color(bg_lerp(4));
        result[6] = format!("   {} {}", "Author: ".custom_color(bg_lerp(6)), "Tomasz Jaworski".custom_color(logo_lerp(6)));
        result[7] = format!("   {} {}", "Version:".custom_color(bg_lerp(7)), env!("CARGO_PKG_VERSION").custom_color(logo_lerp(7)));

        result[11] = "Supported Non-UCI Commands".custom_color(bg_lerp(11));
        result[13] = "   draw".custom_color(logo_lerp(13));
        result[14] = "   clear".custom_color(logo_lerp(14));
        result[15] = "   tree  <depth> <node_idx>".custom_color(logo_lerp(15));
        result[16] = "   perft <depth>".custom_color(logo_lerp(16));
        result[17] = "   bulk  <depth>".custom_color(logo_lerp(17));
        result[18] = "   bench <depth>".custom_color(logo_lerp(18));
        result[19] = "   eval".custom_color(logo_lerp(19));
        result[20] = "   policy".custom_color(logo_lerp(20));

        result
    };

    for (line, info) in logo.into_iter().zip(info) {
        result.push_str(format!("{line}    {info}\n").as_str());
    }

    result
}