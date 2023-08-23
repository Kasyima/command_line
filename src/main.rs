//! It's features will be::
//! Take a message string as the positional argument.
//! Take a -h/--help flag to print help message
//! Take a -d/--dead flag that makes the cat's eyes become xx, which is
//! the comical expression of dead eyes.
//!
//! Print in color
//! Print the error messages to STDERR for error handling.
//! Accept STDIN for piping and pipe the output to other programs..
//! Perform integration tests.
//! Package and publish to crates.io.
//!
//! cargo add clap --features derive

use std::io::{self, Read};

use anyhow::{Context, Result};
use colored::Colorize;

use clap::Parser;
#[derive(Parser)] // ? The derive feature adds a derive macro that automatically
                  // ? generates some parsing code on any struct.
                  // ? A macro defined by the clap crate automatically generates the required parser code for
                  // ? the struct from command-line arguments.
                  // ? The parser outputs the parsed arguments in the struct format you defined.
                  // ? The struct is annotated with the custom derive attribute, indicating the struct is our command-line arguments definition.
                  // ? The Parser derive macro generates the argument parser accordingly.

struct Options {
    #[clap(default_value = "Meow!")]
    // ? The default value is set by annotating the field with
    // ? #[clap(default_value = "Meow!")]
    /// What does the cat say?
    // ? Clap uses it as the description for that field.
    message: String,

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool, // ? When a flag has the bool type, its value is determined by its presence.
    // ? If the flag is present, its value will be true, otherwise, false.
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>, // ? Inside the Option<T> we use a std::path::PathBuf instead of a
    // ? raw string. PathBuf can help us handle paths to files more robustly because it hides away many
    // ? differences in how the OS represent paths.
    #[clap(short = 'i', long = "stdin")]
    stdin: bool,
}

fn main() -> Result<()> {
    /*
    let message = std::env::args()
        .nth(1)
        .expect("Missing the message. Usage: catsay <message>");
    */

    let options = Options::parse();
    // ? Options::parse(), returns an Options struct populated with the passed argument values.

    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    let eye = if options.dead { "x" } else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;
            // ? Use std::fs::read_to_string(path) to read the file contents to a string.
            // ? The ? operator performs a match on the Result returned by read_to_string().
            // ? If the value is Ok(). it unwraps the value inside it.
            // ? If the value is Err(), the function early returns with the error wrapped inside, possibly
            // ? converted into the error type of the wrapping function.

            // ? This is useful if you have multiple potential points of failure in your function.

            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture.bright_green().on_bright_magenta());
        }
        None => {
            // ... print the cat as before
        }
    }
    println!("{}", message.bright_magenta().underline().on_bright_cyan());
    println!(" \\");
    println!("  \\");
    println!("     /\\_/\\");
    println!(
        "    ( {eye} {eye} )",
        eye = eye.blink().bright_yellow().bold()
    );
    println!("    =( I )=");

    Ok(())
}

/*

* The colored crate defines a Colorize trait, which is implemented on a &str and String.
* This trait provides various chainable coloring functions::
    * Coloring the text: .red(), .green(), .blue(), etc.
    * Coloring the background: .on_red(), .on_green(), .on_blue(), etc.
    * Brighter version: .bright_red(), .on_bright_green(), etc.
    * Styling: .bold(), .underline(), .italic(), etc.

? If you want to define a more user-friendly error, you can use the anyhow crate



- The anyhow crate provides a Context trait, which wraps the original error with a human-readable and
user-facing explanation of the error, called context.
- To use Context, you can call the with_context() function on std::result::Result so you can define a context
message.
- The Result from std is shadowed by anyhow::Result.
- This struct contains the std::io::Error from read_to_string() and the "could not read file filename" error message.

- The anyhow crate allows you to ignore the specifics of what error type you want to use, and easily put all the info'
in a single Result type with different context data.
- The anyhow crate and its context are much more than just printing a human-friendly error message.
- If we have a function call chain, we can have a nested chain of errors; each has a context that is relevant to the
layer of abstraction.
- It also gives ergonomic ways to manipulate the chain of errors and backtraces.
- It allows easier downcasting from a generic Error to a specific type, compared to the built-in std::error::Error.



! Piping to Other Commands

- Piping -- the text output of one command can be sent directly to another command as an input.

- This allows a command-line program to be designed in a modular way and multiple programs to work
- together easily without specialized protocols.

- You can set the NO_COLOR environment variable to 1 to turn off the coloring.


! Accepting STDIN

- Taking input from STDIN is another way to interact with other programs.
- U can create a switch --stdin that enables this behavior.

- Eg.., echo -n "Hello world" | catsay --stdin
- The -n flag for echo is to tell echo not to add a new line to the end of its STDOUT output.



*/
