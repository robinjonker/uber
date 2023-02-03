use log::{LevelFilter, Level};
use pretty_env_logger::env_logger::{fmt::{Color, Style, StyledValue}, Target};
use std::fmt;
use chrono::Local;

struct Padded<T> {
    value: T,
    width: usize,
}

impl<T: fmt::Display> fmt::Display for Padded<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{: <width$}", self.value, width=self.width)
    }
}
fn get_color(level: &Level) -> Color {
    match level {
        Level::Trace => Color::Magenta,
        Level::Debug => Color::Blue,
        Level::Info =>Color::Green,
        Level::Warn => Color::Yellow,
        Level::Error => Color::Red,
    }
}

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
    }
}

fn colored_prompt<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.value("🤫"),
        Level::Debug => style.value("🗣️"),
        Level::Info => style.value("🔈"),
        Level::Warn => style.value("⚠"),
        Level::Error => style.value("😱"),
    }
}

pub fn setup() {
    let _ = pretty_env_logger::formatted_builder()
        .target(Target::Stdout)
        .format(|f, record| {
            use std::io::Write;
            let filename = record.file().unwrap_or_default();
            let max_width = 38;
            let mut style = f.style();
            let level = colored_level(&mut style, record.level());

            let mut style = f.style();
            let prompt = colored_prompt(&mut style, record.level());

            let mut style = f.style();

            let line_number = record.line().unwrap_or_default().to_string();
            let trancated_filename = if filename.len()  > (max_width - line_number.len()) {
                &filename[(filename.len() - max_width + line_number.len())..]
            }else{
                filename
            };
            let parts = trancated_filename.split("/crafty/src").collect::<Vec<_>>();
            let trancated_filename=  if parts.len() > 1 {
                format!("/crafty/src{}", parts[1])
            }else{
                trancated_filename.to_string()
            };


            let fileline_number = style.set_bold(false).value(Padded {
                value: format!("{}:{}", trancated_filename, record.line().unwrap_or_default()),
                width: max_width + 1,
            });
            let mut style = f.style();
            let styled_time = style.set_color(get_color(&record.level()))
                .value(Local::now().format("%Y-%m-%dT%H:%M:%S"));

            writeln!(
                f,
                "[{}] {} |{}| {} {}",
                level,
                fileline_number,
                styled_time,
                prompt,
                record.args(),
            )
        })
        .filter(None, LevelFilter::Info)
        .filter(Some("tonic"), LevelFilter::Info)
        .filter(Some("crafty"), LevelFilter::Info)
        .init();
}
