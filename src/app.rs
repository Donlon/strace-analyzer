/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 *                                                                           *
 *  Copyright  (C)  2015-2020  Christian Krause                              *
 *                                                                           *
 *  Christian Krause  <christian.krause@mailbox.org>                         *
 *                                                                           *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
 *                                                                           *
 *  This file is part of strace-analyzer.                                    *
 *                                                                           *
 *  strace-analyzer is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by     *
 *  the Free Software Foundation, either version 3 of the license, or any    *
 *  later version.                                                           *
 *                                                                           *
 *  strace-analyzer is distributed in the hope that it will be useful, but   *
 *  WITHOUT ANY WARRANTY; without even the implied warranty of               *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU         *
 *  General Public License for more details.                                 *
 *                                                                           *
 *  You should have received a copy of the GNU General Public License along  *
 *  with strace-analyzer. If not, see <http://www.gnu.org/licenses/>.        *
 *                                                                           *
 * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use atty::Stream;
use clap::{crate_description, crate_name, crate_version};
use clap::{App, AppSettings, Arg};
use std::path::Path;

use crate::output::Output;

pub fn build() -> App<'static, 'static> {
    let color = atty::is(Stream::Stdout);

    let color = if color {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    let format = Arg::with_name("format")
        .long("format")
        .help("output format")
        .long_help(
"Specify output format of the report. `prometheus` uses the Prometheus' \
 metric exposition format. `oneline` is intended as machine-readable output \
 that shows a colon (\":\") separated list of age, total, accessed, and \
 modified size in bytes, followed by the directory. `table` (cargo feature, \
 enabled by default) shows a pretty-printed table."
        )
        .takes_value(true)
        .case_insensitive(true)
        .possible_values(&Output::variants())
        .display_order(1);

    let format = if cfg!(feature = "table") {
        format.default_value("table")
    } else {
        format.default_value("continuous")
    };

    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .after_help("create traces with: strace -s 0 -ff -o cmd.strace cmd")
        .global_setting(color)
        .max_term_width(80)
        .help_short("?")
        .help_message("show this help output")
        .version_message("show version")
        .arg(&format)
        .arg(
            Arg::with_name("file")
                .help("strace output file name")
                .long_help(
                    "The primary output file name of the strace run. \
                     strace-analyzer will follow other strace files \
                     created via the strace -ff flag. The followed files \
                     are determined based on the clone syscalls that are \
                     encountered in the traces.",
                )
                .required(true)
                .validator(is_file),
        )
        .arg(Arg::with_name("debug").long("debug").help("debug output"))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("verbose output"),
        )
}

fn is_file(s: String) -> Result<(), String> {
    let path = Path::new(&s);

    if !path.exists() {
        Err(format!("does not exist: {:?}", path))
    } else if !path.is_file() {
        Err(format!("is not a file: {:?}", path))
    } else {
        Ok(())
    }
}
