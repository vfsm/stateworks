mod stateworks_word_counter;
use stateworks_word_counter::StateMachine;

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
    let mut sm = StateMachine::init();
    // TODO the user must implement only that below. The complete function is a wrapper on
    // the user implemented callback
    for c in text.chars() {
        sm.send_char(c);
    }
    sm.read_counter()
}

// the state machine will be implemented as a lib. No necessity for main.
// Also, TODO we need to think about how it will be implemented: as a separate crate or the
// code will be integrated to the current crate, or both?
fn main() {
    let mut sm = StateMachine::init();
    sm.read_counter();
    sm.send_char('a');
    sm.read_counter();
    sm.send_char('b');
    sm.send_char('b');
    sm.send_char('b');
    sm.send_char('c');
    sm.read_counter();
    sm.send_char('\n');
    sm.read_counter();
    sm.send_char('2');
    sm.send_char('3');
    sm.read_counter();
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

                #[test]
                fn should_count_1000_words() {
                    let lorem_ipsum = include_str!("./lorem_ipsum.txt");
                    assert_eq!($func(lorem_ipsum), 1000);
                }
            }
        };
    }

    test_word_counter!(test_word_counter_reference, word_counter_reference);
    test_word_counter!(test_word_counter_state_machine, word_counter_state_machine);
}
