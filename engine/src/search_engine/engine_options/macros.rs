#[macro_export]
macro_rules! create_options {
    (
        $name:ident {
            Options {
                $(
                    [$option_key:literal] $option:ident : $option_ty:ty =>
                        $option_default:expr $(, $option_min:expr, $option_max:expr)?;
                )+
            }
            Tunables {
                $(
                    $tunable:ident : $tunable_ty:ty =>
                        $tunable_default:expr,
                        $tunable_min:expr,
                        $tunable_max:expr,
                        $tunable_c:expr,
                        $tunable_r:expr;
                )+
            }
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($option: $option_ty,)+
            $($tunable: $tunable_ty,)+
        }

        impl $name {
            pub fn new() -> Self {
                Self {
                    $($option: $option_default,)+
                    $($tunable: $tunable_default,)+
                }
            }

            $(
            pub fn $option(&self) -> $option_ty {
                self.$option.clone()
            }
            )+

            $(
            pub fn $tunable(&self) -> $tunable_ty {
                self.$tunable.clone()
            }
            )+

            pub fn set_option(&mut self, name: &str, value: &str) -> bool {
                match name {
                    $(
                    $option_key => {
                        match value.parse::<$option_ty>() {
                            Ok(new_value) => {
                                $(
                                if new_value < $option_min || new_value > $option_max {
                                    eprintln!("Value out of range for {}", name);
                                    return false;
                                }
                                )?

                                if self.$option == new_value {
                                    eprintln!("Value of {} is already {}", name, new_value);
                                    return false;
                                }

                                self.$option = new_value;
                                true
                            }
                            Err(_) => {
                                eprintln!("Incorrect param type for {}", name);
                                false
                            }
                        }
                    }
                    )+

                    $(
                    stringify!($tunable) => {
                        match value.parse::<$tunable_ty>() {
                            Ok(new_value) => {
                                if new_value < $tunable_min || new_value > $tunable_max {
                                    eprintln!("Value out of range for {}", name);
                                    return false;
                                }

                                if self.$tunable == new_value {
                                    eprintln!("Value of {} is already {}", name, new_value);
                                    return false;
                                }

                                self.$tunable = new_value;
                                true
                            }
                            Err(_) => {
                                eprintln!("Incorrect param type for {}", name);
                                false
                            }
                        }
                    }
                    )+

                    _ => {
                        eprintln!("Unknown option '{}'", name);
                        false
                    }
                }
            }

            pub fn print_options(&self) {
                $(
                {
                    let uci_type = match stringify!($option_ty) {
                        "bool" => "check",
                        "i64"  => "spin",
                        _      => "string",
                    };

                    let mut default_str = $option_default.to_string();
                    if default_str.is_empty() {
                        default_str = "<empty>".to_string();
                    }

                    print!(
                        "option name {} type {} default {}",
                        $option_key, uci_type, default_str
                    );

                    $( print!(" min {} max {}", $option_min, $option_max); )?
                    println!();
                }
                )+
            }

            pub fn print_tunables(&self) {
                $(
                {
                    let kind = if stringify!($tunable_ty) == "i64" { "int" } else { "float" };
                    println!(
                        "{}, {}, {}, {}, {}, {}, {}",
                        stringify!($tunable), kind,
                        self.$tunable, $tunable_min, $tunable_max,
                        $tunable_c, $tunable_r
                    );
                }
                )+
            }
        }
    };
}
