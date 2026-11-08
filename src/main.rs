// ferrissay, a cowsay remake using Rust's mascot "Ferris", written in Rust, obviously
// Copyright (c) 2026 ghlilith1k

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
    // STRUCTS

    // Ferris structure
    struct Ferris<'a> {
        // every word/argument the user wrote.
        words: Vec<String>,
        // printed text box phrase containing all words the user wrote.
        text_box_message: String,
        // printed top text box lines.
        text_box_lines_top: String,
        // printed bottom text box lines.
        text_box_lines_bottom: String,
        // text box lines char counter for pretty formatting.
        text_box_lines_count: u32,
        // check user declared claws-down argument.
        claws_up_check: bool,
        // the mighty Ferris himself.
        claws_down: &'a str,
        // same guy, just with his claws up in the air.
        claws_up: &'a str,
    }

    // VARIABLES

    // every words the user wrote.
    let mut ferris = Ferris {
        words: env::args().collect(),
        text_box_message: String::new(),
        text_box_lines_bottom: String::new(),
        text_box_lines_top: String::new(),
        text_box_lines_count: 0,
        claws_up_check: false,
        claws_down: r"
        *
         * /^^^^^\
          /  0  0 \
         \\____v__//
          \\<   >//",
        claws_up: r"
        *
       U * /^^^^^\   U
        \_/  0  0 \_/
          \ ___v__/
          \\     //",
    };

    // MAIN LOGICS AND FORMATTING

    // check if atleast one word is present, if not, print help message and terminate program early.
    if ferris.words.len() == 1 {
        println!(
            "ferrissay v0.1.1-beta, (c) 2026 ghlilith1k\nsee https://github.com/ghlilith1k/ferrissay for details\n\x20usage: ferrissay [-u] [message]"
        );
        return;
    }

    // checks if user explicited claws up Ferris, if yes, switch to claws up and delete flag from arguments.
    if ferris.words[1].contains("-u") {
        ferris.claws_up_check = true;
        ferris.words.remove(1);
    }

    // removes first argument since it's not supposed to get printed.
    ferris.words.remove(0);

    // this makes all words getting printed without using quotes.
    for word in ferris.words {
        ferris.text_box_message.push_str(&format!("{} ", word))
    }

    // counter for text box lines pretty formatting, without it, the lines goes over the ">".
    for _ in ferris.text_box_message.chars() {
        ferris.text_box_lines_count += 1
    }

    // pushes all text boxt lines chars minus one for pretty formatting.
    for _ in 0..ferris.text_box_lines_count - 1 {
        ferris.text_box_lines_top.push_str("_");
        ferris.text_box_lines_bottom.push_str("r")
    }

    // PRINTING

    // very confusing formatting, isn't it?
    if !ferris.claws_up_check {
        println!(
            " _{}_\n< {}>\n -{}-{}",
            ferris.text_box_lines_top,
            ferris.text_box_message,
            ferris.text_box_lines_bottom,
            ferris.claws_down
        )
    } else {
        println!(
            " _{}_\n< {}>\n -{}-{}",
            ferris.text_box_lines_top,
            ferris.text_box_message,
            ferris.text_box_lines_bottom,
            ferris.claws_up
        )
    }
}
