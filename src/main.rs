// ferrissay, a cowsay remake using Rust's mascot "Ferris", written in Rust, obviously.
// Copyright (C) 2026 ghlilith1k

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// DEPENDENCIES
use std::env;

// MAIN
fn main() {
    
    // VARIABLES
    // every words the user wrote.
    let mut args: Vec<String> = env::args().collect();
    // printed text box phrase containing all words the user wrote.
    let mut ferris_text_box_message: String = String::new();
    // printed top text box lines.
    let mut ferris_text_box_lines_top: String = String::new();
    // printed bottom text box lines.
    let mut ferris_text_box_lines_bottom: String = String::new();
    // text box lines char counter for pretty formatting.
    let mut ferries_text_box_lines_count: u32 = 0;
    // the mighty Ferris himself, with his claws up in the air.
    let ferris_claws_up: &str =
    r"          \
       U   /^^^^^\   U
        \_/  0  0 \_/
          \ ___v__/
          \\     //"
    ;
    
    // MAIN LOGICS
    // check if atleast one word is present, if not, terminate program early.
    if args.len() == 1 {
        println!("usage: ferrissay [something...]");
        return()
    }
    // removes first argument since it's not supposed to get printed
    args.remove(0);
    // this makes all words getting printed without using quotes.
    for arg in args {
        ferris_text_box_message.push_str(&format!("{} ", arg))
    }
    // counter for text box lines pretty formatting, without it, the lines goes over the ">".
    for _ in ferris_text_box_message.chars() {
        ferries_text_box_lines_count += 1
    }
    // pushes all text boxt lines chars minus one for pretty formatting.
    for _ in 0..ferries_text_box_lines_count -1 {
        ferris_text_box_lines_top.push_str("_");
        ferris_text_box_lines_bottom.push_str("-")
    }
    
    // FORMATTING AND PRINTING
    // very confusing formatting, isn't it?
    println!(" _{}_\n< {}>\n -{}-\n{}",
        ferris_text_box_lines_top,
        ferris_text_box_message,
        ferris_text_box_lines_bottom,
        ferris_claws_up
    )

}
