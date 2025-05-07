mod stateworks {
    //! This module contains a very naive and basic implementation of a state machine
    //! structure that will allow the user to count words. There will be no worry about
    //! optimizations at the moment.
    //!
    //! The model to be implemented is a very basic homogeneous iterative synchronous state
    //! machine.
    //! The meaning is the following:
    //! - Homogeneous: the user will only be able to send one type of input to the machine,
    //! in this case, chars.
    //! - Iterative: the state machine will work on an iterative fashion. Each iteration,
    //! the user will send a new char to it.
    //! - Synchronous: each time the user sends an input to the machine, the program must
    //! wait for it to finish the calculations for that iteration before continuing to the
    //! next command.
    //!
    //! That limitations will make it much simpler to implement. Later, we try to add more
    //! complexity and optimizations.

    use std::collections::HashMap;

    struct Action;
    struct Condition;

    struct Always {
        actions: Vec<Action>,
    }

    struct State<'a> {
        name: &'a str,
        entry_actions: Vec<Action>,
        exit_actions: Vec<Action>,
        input_actions: HashMap<Condition, Action>,
        transitions: HashMap<Condition, State<'a>>,
    }

    struct StateMachine<'a> {
        states: Vec<State<'a>>,
    }

    /*
    struct RTDB{}
    */

    // In the future, our intention is that all of the code generated to run the state
    // machine will be automatically generated, and it will not be intended to edit it
    // directly. Only the high level specification of the state machine should be edited.
    impl<'a> StateMachine<'a> {
        fn new() -> Self {
            let init = State {
                name: "init",
                entry_actions: vec![],
                exit_actions: vec![],
                input_actions: HashMap::new(),
                transitions: HashMap::new(),
            };
            // TODO continue from here...
            StateMachine { states: vec![init] }
        }
    }
}

/// Counts the number of words in a given string. Each word is separated by white spaces.
///
/// This implementation will be used as reference for the state machine implementation.
fn word_counter_reference(text: &str) -> u32 {
    text.split_whitespace().count() as u32
}

/**
 * There are some considerations to do about this use case. First, we need to define the IO
 * interface between the programming language and the state machine. The interface will be
 * the signature of the function. Thus, our state machine will receive a text and will
 * return an u32.
 */
fn word_counter_state_machine(text: &str) -> u32 {
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_word_counter {
        ($name:ident, $func:ident) => {
            mod $name {
                use super::*;

                #[test]
                fn should_count_0_words() {
                    assert_eq!($func(""), 0);
                    assert_eq!($func("    "), 0);
                    assert_eq!($func("\n\t"), 0);
                }

                #[test]
                fn should_count_1_words() {
                    assert_eq!($func("i"), 1);
                    assert_eq!($func("  hi  "), 1);
                    assert_eq!($func("\n\thello    \n\n"), 1);
                }

                #[test]
                fn should_count_2_words() {
                    assert_eq!($func("i a"), 2);
                    assert_eq!($func("  hi there  "), 2);
                    assert_eq!($func("\n\thello\nworld\n"), 2);
                }

                #[test]
                fn should_count_5_words() {
                    assert_eq!($func("This text has 5 words.\n"), 5);
                }
            }
        };
    }

    test_word_counter!(test_word_counter_reference, word_counter_reference);
    test_word_counter!(test_word_counter_state_machine, word_counter_state_machine);
}
