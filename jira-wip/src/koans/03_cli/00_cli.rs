/// There are many ways to expose the functionality we built to a user: an API, a GUI, etc.
/// We will go with something simpler, yet good enough to probe at our implementation
/// and touch with our own hands the fruit of our labor: a command line application, a CLI.
///
/// Rust is well-equipped to write CLIs: we will be using `structopt`, a crate
/// that provides a derive macro to define a CLI interface declaratively.
///
/// We define the structure of our commands, annotating each field appropriately,
/// and `#[derive(structopt::StructOpt)]` takes care of generating all the code
/// required to parse the user input as well as generating a detailed `--help` page
/// for the CLI itself and each of its subcommands.
///
/// Comments on each of the field and each of the `Command` variant will be shown in the
/// help page of those commands!
///
/// You can learn more about `structopt` looking at their documentation: https://docs.rs/structopt/0.3.12/structopt/
/// You can see the code generated by `structopt` using `cargo expand`, see https://github.com/dtolnay/cargo-expand
///
/// Fill in the missing fields!
///
/// When you are ready, uncomment the appropriate lines from src/main.rs and
/// run `cargo run --bin jira-wip` in your terminal!
pub mod cli {
    use super::store_recap::{TicketStore, Status, TicketDraft, TicketPatch, TicketTitle, TicketDescription};
    use super::id_generation::TicketId;
    use std::error::Error;
    use std::str::FromStr;
    use std::fmt::Formatter;

    #[derive(structopt::StructOpt, Clone)]
    /// A small command-line interface to interact with a toy Jira clone, IronJira.
    pub enum Command {
        /// Create a ticket on your board.
        Create {
            __
        },
        /// Edit the details of an existing ticket.
        Edit {
            /// Id of the ticket you want to edit.
            #[structopt(long)]
            id: TicketId,
            /// New status of the ticket.
            #[structopt(long)]
            status: Option<Status>,
            /// New description of the ticket.
            #[structopt(long)]
            description: Option<TicketDescription>,
            /// New title for your ticket.
            #[structopt(long)]
            title: Option<TicketTitle>,
        },
        /// Delete a ticket from the store passing the ticket id.
        Delete {
            __
        },
        /// List all existing tickets.
        List,
    }

    /// `structopt` relies on `FromStr` to know how to parse our custom structs and enums
    /// from the string passed in as input by a user.
    ///
    /// Parsing is fallible: we need to declare what error type we are going to return if
    /// things go wrong and implement the `from_str` function.
    impl FromStr for Status {
        type Err = ParsingError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            __
        }
    }

    impl FromStr for TicketTitle {
        __
    }

    impl FromStr for TicketDescription {
        __
    }

    /// Our error struct for parsing failures.
    #[derive(Debug)]
    pub struct ParsingError(String);

    impl Error for ParsingError { }

    impl std::fmt::Display for ParsingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.0)
        }
    }

    /// The core function: given a mutable reference to a `TicketStore` and a `Command`,
    /// carry out the action specified by the user.
    /// We use `Box<dyn Error>` to avoid having to specify the exact failure modes of our
    /// top-level handler.
    ///
    /// `dyn Error` is the syntax of a trait object, a more advanced topic that we will not be
    /// touching in this workshop.
    /// Check its section in the Rust book if you are curious: https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
    pub fn handle_command(ticket_store: &mut TicketStore, command: Command) -> Result<(), Box<dyn Error>> {
        match command {
            Command::Create { description, title } => {
                __
            }
            Command::Edit {
                id,
                title,
                description,
                status,
            } => {
                __
            }
            Command::Delete { ticket_id } => match ticket_store.delete(&ticket_id) {
                Some(deleted_ticket) => println!(
                    "The following ticket has been deleted:\n{:?}",
                    deleted_ticket
                ),
                None => println!(
                    "There was no ticket associated to the ticket id {:?}",
                    ticket_id
                ),
            },
            Command::List => {
                __
            }
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn invalid_status_fails_to_be_parsed()
        {
            let invalid_status = "Not a good status";
            assert!(Status::from_str(invalid_status).is_err());
        }
    }
}
