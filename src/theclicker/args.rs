use clap::Parser;

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Run {
        #[arg(short = 'd')]
        device_query: String,

        #[arg(short)]
        left_bind: Option<u16>,

        #[arg(short)]
        middle_bind: Option<u16>,

        #[arg(short)]
        right_bind: Option<u16>,

        #[arg(short = 'T')]
        lock_unlock_bind: Option<u16>,

        #[arg(short = 'H', default_value_t = false)]
        hold: bool,

        #[arg(long, default_value_t = false)]
        grab: bool,

        #[arg(short, default_value_t = 25)]
        cooldown: u64,

        #[arg(short = 'C', default_value_t = 0)]
        cooldown_press_release: u64,
    },
    RunLegacy {
        #[arg(short = 'd')]
        device_query: String,

        #[arg(short, default_value_t = 25)]
        cooldown: u64,

        #[arg(short = 'C', default_value_t = 0)]
        cooldown_press_release: u64,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    #[arg(long, default_value_t = false)]
    pub beep: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}
