// MIT License
//
// Copyright (c) 2023 Mansoor Ahmed Memon
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::io;

use konfigurator::cli;
use konfigurator::cli::actions::BAKE;

fn main() -> io::Result<()> {
    let matches = cli::interface().get_matches();

    match matches.subcommand() {
        Some((BAKE, arg_matches)) => {
            let work_dir = arg_matches.get_one::<String>("work_dir");
            let out_dir = arg_matches.get_one::<String>("out_dir");

            match konfigurator::bake(work_dir, out_dir) {
                Ok(out_file) => {
                    println!("Configuration baked successfully: {}", out_file);
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        _ => {
            cli::interface()
                .print_long_help()
                .expect("Fatal: help could not be displayed.");
        }
    }

    Ok(())
}
