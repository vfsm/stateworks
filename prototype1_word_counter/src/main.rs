//use stateworks::word_counter;

/*
Basic design for this prototype.
Given that the application of state machines to software design is not so obvious as for hardware, I felt the necessity to make a basic design strategy before implementing this prototype. Upon starting the implementation, I noticed how little I know about state machines as systems for controlling the state of the program. I found a lot of situations where multiple choices could have been made, and that made the design very confusing. Thus, before starting the implementation, I will start with a basic design for the prototype.

First, the main goals for this prototype:
1. Allow the use of state machine to control a word counter function.
2. Improve my understanding of state machines applied to software engineering.
3. Separate the control flow, or part of it, from the data flow. The control flow will be handled by the state machine. The data flow, will be handled by the user-functions and by the other IO-objects like counters, timers, etc.

We are going to start with a Function based state machine. That means that the programmer will only interface with the state machine through the state machine function.

In our example, the function will be `fn word_counter(text:&str)->u32`. The state machine will be instantiated only inside the function and the programmer will not interact directly with it, only with its function.


For this prototype, we are going to need the following items:
1 - A counter to store the number of words counted until the moment
2 - State machine for the word counter
3 - Definition on how the VI and VO will be handled
4 - An execution model for the state machine and all the objects of the RTDB

# State Machine
Each state machine will consist of a set of states. Each state will be associated with a table that defines (1) - input actions associated with conditions, (2) - transition actions (although the book suggest not using it, I think it may make the design easier and clearer sometimes), (3) - Entry actions, (4) - Exit actions, (5) - transitions, describing the condition, the next state and the actions that will be taken.

# Virtual Inputs
We may have multiple types of VI. For example: one-time signal, static signal, limited time signals (signals that may have limited time/cycles of use in the execution model). But, for this prototype, only two of them will be used: one-time and static signals. One time signals will be consumed after, at most, the first state transition (we may think and investigate the future possibility of one-time signals that last multiple state transitions, but that may not be something useful), and if no state transition was done, they will be consumed when the execution returns to idle state. Static signals will last until they are changed.

About static signals, we may have two possible approaches, first, allow their use for input actions but only when they change (equivalent to a change of state which generates one-time signal), and, second, does not allow them in input actions. I think it is a good idea to allow them in the input action.

# Execution model


# Draft: important aspects to discuss and remember during implementation

1. Be careful to avoid an implementation of a pure event driven design. I do not want to code the state of the variables into the state of the state machine.

*/

use prototype_stateworks::StateMachine;

mod prototype_stateworks {
    use std::{
        collections::{HashMap, HashSet},
        fmt::Display,
        sync::OnceLock,
    };

    // First, let's build a very simple state machine
    // It will have a programming interface to receive a stream of chars and, for each char,
    // it will generate some VI value that will make the state machine update its state

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum VirtualInputType {
        Event,
        StaticSignal,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum VirtualInput {
        // IO-Object: CharStreamInput
        ReadAlphanumeric,
        ReadOther,

        // Always static signal
        Always,
    }
    impl VirtualInput {
        fn r#type(&self) -> VirtualInputType {
            match self {
                VirtualInput::ReadAlphanumeric => VirtualInputType::Event,
                VirtualInput::ReadOther => VirtualInputType::Event,
                VirtualInput::Always => VirtualInputType::StaticSignal,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum VirtualOutput {
        PrintHello,

        // IO-Object: Counter
        IncrementCounter,
    }

    fn print_hello() {
        println!("Hello world from state machine");
    }

    #[derive(Debug, Clone)]
    struct Condition(HashSet<VirtualInput>);

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum State {
        Init,
        InWord,
        OutWord,
    }

    #[derive(Debug, Clone)]
    struct StateSpec {
        name: &'static str,
        entry_actions: Vec<VirtualOutput>,
        exit_actions: Vec<VirtualOutput>,
        input_actions: Vec<(Condition, VirtualOutput)>,
        /// The last element of the tuple is the array of transition actions
        transitions: Vec<(Condition, State, Vec<VirtualOutput>)>,
    }

    #[derive(Debug)]
    pub struct StateMachine {
        current_state: State,
        virtual_input: HashSet<VirtualInput>,
        // TODO maybe creating a StateMachine struct as the top level is not the best approach. Create an
        // RTDB/VFSM struct may be better. Every IO object and state machine may be included
        // to this VFSM. I don't know, something like that.
        counter: u32,
    }
    static STATES: OnceLock<HashMap<State, StateSpec>> = OnceLock::new();
    static GLOBAL_INPUT_ACTIONS: OnceLock<Vec<(Condition, VirtualOutput)>> = OnceLock::new();

    fn get_global_input_actions() -> &'static Vec<(Condition, VirtualOutput)> {
        &GLOBAL_INPUT_ACTIONS.get_or_init(|| {
            vec![(
                Condition(HashSet::from_iter(vec![VirtualInput::Always])),
                VirtualOutput::PrintHello,
            )]
        })
    }

    fn get_state_spec(state: State) -> &'static StateSpec {
        &STATES.get_or_init(|| {
            HashMap::from_iter(vec![
                (
                    State::Init,
                    StateSpec {
                        name: "init",
                        entry_actions: vec![],
                        exit_actions: vec![],
                        input_actions: vec![],
                        transitions: vec![(
                            Condition(HashSet::from_iter(vec![VirtualInput::Always])),
                            State::OutWord,
                            vec![],
                        )],
                    },
                ),
                (
                    State::OutWord,
                    StateSpec {
                        name: "out_word",
                        entry_actions: vec![],
                        exit_actions: vec![],
                        input_actions: vec![],
                        transitions: vec![(
                            Condition(HashSet::from_iter(vec![VirtualInput::ReadAlphanumeric])),
                            State::InWord,
                            vec![VirtualOutput::IncrementCounter],
                        )],
                    },
                ),
                (
                    State::InWord,
                    StateSpec {
                        name: "in_word",
                        entry_actions: vec![],
                        exit_actions: vec![],
                        input_actions: vec![],
                        transitions: vec![(
                            Condition(HashSet::from_iter(vec![VirtualInput::ReadOther])),
                            State::OutWord,
                            vec![],
                        )],
                    },
                ),
            ])
        })[&state]
    }
    impl Display for State {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let name = match self {
                State::Init => "Init",
                State::InWord => "Alphanumeric",
                State::OutWord => "Other",
            };
            write!(f, "{name}",)
        }
    }
    impl Display for StateMachine {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "-------------------------------------------\n")?;
            write!(f, "|              State Machine              |\n")?;
            write!(f, "|State: {:<34}|\n", self.current_state.to_string())?;
            write!(f, "|Virtual Input: {:?}\n", self.virtual_input)?;
            write!(f, "-------------------------------------------")
        }
    }
    impl StateMachine {
        pub fn init() -> Self {
            let mut state_machine = StateMachine {
                virtual_input: HashSet::new(),
                current_state: State::Init,
                counter: 0,
            };
            // initiate the state machine
            state_machine.execute();
            state_machine
        }
        fn accepts_condition(&self, condition: &Condition) -> bool {
            let condition = &condition.0;
            condition.contains(&VirtualInput::Always) || condition.is_subset(&self.virtual_input)
        }

        fn set_state(&mut self, next_state: State) {
            self.current_state = next_state;
        }

        fn execute(&mut self) {
            println!("State machine before:\n{self}");
            let mut current_state = get_state_spec(self.current_state);
            println!("┌─────────────────────────────────────────");
            println!("│ Executing the state machine");
            println!("│ Current state: {:?}", self.current_state);

            println!("├─────────────────────────────────────────");
            println!("│ Checking for input actions:");
            // check if there is any input action

            for (condition, virtual_output) in &current_state.input_actions {
                if self.accepts_condition(condition) {
                    println!("│  ✓ Condition met: {condition:?}");
                    println!("│    Executing: {virtual_output:?}");
                    self.execute_virtual_output(*virtual_output);
                }
            }

            // execute global input actions
            for (condition, virtual_output) in get_global_input_actions() {
                if self.accepts_condition(condition) {
                    println!("│  ✓ Condition met (global): {condition:?}");
                    println!("│    Executing: {virtual_output:?}");
                    self.execute_virtual_output(*virtual_output);
                }
            }

            // Execute every transition
            let mut need_to_check_for_transition;
            println!("├─────────────────────────────────────────");
            println!("│ Checking for transitions:");
            loop {
                need_to_check_for_transition = false;
                for (condition, next_state, transition_actions) in &current_state.transitions {
                    if self.accepts_condition(condition) {
                        println!("│  ✓ Transition triggered: {condition:?} → {next_state:?}");

                        println!("│    ┌─ Executing transition actions");
                        for virtual_output in transition_actions {
                            println!("│    │  • {virtual_output:?}");
                            self.execute_virtual_output(*virtual_output);
                        }

                        // execute exit actions
                        println!("│    ├─ Executing exit actions");
                        for virtual_output in &current_state.exit_actions {
                            println!("│    │  • {virtual_output:?}");
                            self.execute_virtual_output(*virtual_output);
                        }

                        // Make the transition
                        self.set_state(*next_state);
                        current_state = get_state_spec(self.current_state);
                        println!("│    │");
                        println!("│    └─ State changed to: {next_state:?}");

                        println!("│       ┌─ Executing entry actions");
                        for virtual_output in &current_state.entry_actions {
                            println!("│       │  • {virtual_output:?}");
                            self.execute_virtual_output(*virtual_output);
                        }
                        println!("│       └─────────────────────────");
                        need_to_check_for_transition = true;
                        break;
                    }
                }
                if !need_to_check_for_transition {
                    // consume the one-time events (they only live until up to the first
                    // transition)
                    self.consume_events();
                    break;
                }
                // consume the one-time events (they only live until up to the first
                // transition)
                self.consume_events();
            }
            println!("└─────────────────────────────────────────");
            println!("State machine returned to idle\n{self}");
        }

        fn execute_virtual_output(&mut self, vo: VirtualOutput) {
            println!("Executing: {vo:?}");
            match vo {
                VirtualOutput::PrintHello => print_hello(),
                VirtualOutput::IncrementCounter => self.increment_counter(),
            }
        }

        fn consume_events(&mut self) {
            // This may be improved in the future separating Events from static signals
            self.virtual_input
                .retain(|e| e.r#type() != VirtualInputType::Event);
        }

        fn emit_events(&mut self, events: Vec<VirtualInput>) {
            assert!(
                events.iter().all(|e| e.r#type() == VirtualInputType::Event),
                "emit_events only accept events"
            );
            self.virtual_input.extend(events);
        }

        // StreamInputInterface
        // (this function will be implemented and designed, including name, by the user)
        pub fn send_char(&mut self, c: char) {
            println!("#########################################");
            println!("#==> CharInputInterface emitting: '{c}' #");
            println!("#########################################");
            if c.is_alphanumeric() {
                self.emit_events(vec![VirtualInput::ReadAlphanumeric]);
            } else {
                self.emit_events(vec![VirtualInput::ReadOther]);
            }

            // execute only after emitting an event
            self.execute();
        }

        // CounterOutputInterface
        pub fn read_counter(&self) -> u32 {
            println!("(counter = {})", self.counter);
            self.counter
        }

        fn increment_counter(&mut self) {
            self.counter += 1;
        }
    }

    /*
    struct VFSM {}

    impl VFSM {
        // execute a cycle of execution
        fn execute(&self) {
            // Check for input action condition
        }

        fn add_char_to_stream(&self, c: char) {
            if c.is_alphanumeric() {
                // emit ReadAlphanumeric
                return;
            }
            // emit ReadOther
        }
    }
    */
}

mod stateworks {
    /*
    //! This module contains a very naive and basic implementation of a state machine
    //! structure that will allow the user to count words. There will be no worry about
    //! optimizations at the moment.
    //!
    //! The model to be implemented is a very basic passive homogeneous iterative synchronous
    //! function state
    //! machine (HISFSM).
    //! The meaning is the following:
    //! - Passive: it will only execute the VFSM model execution when triggered by the code.
    //! - Homogeneous: the user will only be able to send one type of input to the machine,
    //! in this case, chars.
    //! - Iterative: the state machine will work on an iterative fashion. Each iteration,
    //! the user will send a new char to it.
    //! - Synchronous: each time the user sends an input to the machine, the program must
    //! wait for it to finish the calculations for that iteration before continuing to the
    //! next command.
    //! - Function: the only interface between this state machine and the remaining of the
    //! programming world will be through a function. That may not be the case in the
    //! future, where we may want to implement state machines as structs, allowing more
    //! sophisticated interactions and APIs.
    //!
    //! That limitations will make it much simpler to implement. Later, we try to add more
    //! complexity and optimizations.

    // TODO Comments on VI (those comments will focus only on synchronous state machines):
    // One of the most important aspects of the VI is its signal lifetime. Also, to me, it
    // was one of the most challenging aspects to think about, because we don't have the
    // habit to think about it while developing software. For example, I have never think
    // about what a return value of a function may be considered. Or what a while condition
    // is. Now, while designing the prototype, I am obliged to think about it.
    //
    // Basically, there are three types of signal lifetimes that we may encounter in state
    // machines, followed by my interpretation of them and what it means in the software
    // context:
    // - one-time signal: those will be consumed only once after they are emitted. Once
    // consumed (even if they did not trigger anything), this kind of signal become absent
    // until a new one comes. Their absence cannot be used as signal. As an example, we have
    // a programming function that transforms a programming input into an VI. The
    // programming function may generate any number of families of one-time VI signals.
    //
    // # Example
    // fn parse(char)->ParseResult{ ReadAlphanumeric, ReadOther}
    // fn check_for_even_and_greater_than(input: num, reference: num)->(OddResult{IsOdd, IsEven},
    // GreateResult{IsGreater, IsNotGreater})
    // - static signal: those will not be consumed after they are changed. They will remain
    // the same until their IO-object or item request their change. They will store the last
    // updated value to be used anytime a condition has this kind of signal.
    // counter-> States{ Over, Counting, Init}
    //
    // They may be represented as states of other state machines.
    // struct variableX -> states {Positive, Negative, NotDefined}
    // - limited duration signal: those will not be consumed after they are changed, but
    // they may change

    use std::collections::{HashMap, HashSet};

    // each state machine defines a set of virtual inputs and virtual outputs
    // Both are simply enums:

    // Word counter hard coded implementation

    // For the word counter, all of those VI will be read from the real input of an
    // iterator. In other words, all of them are generated by the action of an input string
    // (from the function input), a preprocessing of this string and transformation of each
    // char into the VI.
    //
    // Who decides when the input will be triggered? the programmer.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum VirtualInput {
        /// Read anything that is not an alphanumeric char.
        ReadOther,
        ReadAlphanumeric,
        // /// Read the last char of the file
        //ReadEOF,
        // TODO: think about it: should I create an Always VI to represent a condition that
        // accepts any substate of the VI set? Or should I implement that in the Condition
        // struct?
        Always,
    }

    // At the moment, I am not worried about the categorization of useful IO-objects. I am
    // only defining the simplest subset necessary for this state machine to work. For
    // example, in this situation, it is interesting to define a generic Counter object,
    // which could be used by any project.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum VirtualOutput {
        IncrementCounter,
        // /// This action will be triggered by any input action of the type ReadEOF.
        // /// After generating this output, the return of the function will be triggered.
        //Return,
    }

    #[derive(Debug, Clone)]
    struct Condition {
        // At the moment, only a bunch of AND conditions will be enough. Also, maybe storing
        // them as vector is not the best choice, because we will make set operations with
        // them. But that is something for the future..
        and: HashSet<VirtualInput>,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum State {
        Init,
        InWord,
        OutWord,
    }

    #[derive(Debug, Clone)]
    struct StateSpec<'a> {
        name: &'a str,
        entry_actions: Vec<VirtualOutput>,
        exit_actions: Vec<VirtualOutput>,
        input_actions: Vec<(Condition, VirtualOutput)>,
        transitions: Vec<(Condition, State)>,
    }

    #[derive(Debug)]
    struct StateMachine<'a> {
        current_state: State,
        states: HashMap<State, StateSpec<'a>>,
        // Input actions that will be executed always, regardless of the current state
        always: Vec<(Condition, VirtualOutput)>,
    }

    // In the future, our intention is that all of the code generated to run the state
    // machine will be automatically generated, and it will not be intended to edit it
    // directly. Only the high level specification of the state machine should be edited.
    impl<'a> StateMachine<'a> {
        fn new() -> Self {
            let init = StateSpec {
                name: "init",
                entry_actions: vec![],
                exit_actions: vec![],
                input_actions: vec![],
                transitions: vec![(
                    Condition {
                        and: HashSet::from_iter(vec![VirtualInput::Always]),
                    },
                    State::InWord,
                )],
            };
            let in_word = StateSpec {
                name: "in_word",
                entry_actions: vec![],
                exit_actions: vec![],
                input_actions: vec![],
                transitions: vec![
                    (
                        Condition {
                            and: HashSet::from_iter(vec![VirtualInput::ReadOther]),
                        },
                        State::OutWord,
                    ),
                    (
                        Condition {
                            and: HashSet::from_iter(vec![VirtualInput::ReadAlphanumeric]),
                        },
                        State::InWord,
                    ),
                ],
            };

            let out_word = StateSpec {
                name: "out_word",
                entry_actions: vec![],
                exit_actions: vec![],
                input_actions: vec![(
                    Condition {
                        and: HashSet::from_iter(vec![VirtualInput::ReadAlphanumeric]),
                    },
                    VirtualOutput::IncrementCounter,
                )],
                transitions: vec![
                    (
                        Condition {
                            and: HashSet::from_iter(vec![VirtualInput::ReadOther]),
                        },
                        State::OutWord,
                    ),
                    (
                        Condition {
                            and: HashSet::from_iter(vec![VirtualInput::ReadAlphanumeric]),
                        },
                        State::InWord,
                    ),
                ],
            };

            StateMachine {
                current_state: State::Init,
                states: HashMap::from_iter(vec![
                    (State::Init, init),
                    (State::InWord, in_word),
                    (State::OutWord, out_word),
                ]),
                always: vec![/*(
                    Condition {
                        and: HashSet::from_iter(vec![VirtualInput::ReadEOF]),
                    },
                    VirtualOutput::Return,
                )*/],
            }
        }

        fn emit_virtual_output(&mut self, vo: VirtualOutput) {}
        // in this case, it is very simple to emit each VI at time, but in the future, may
        // be interesting to investigate the approach of registering the VIs for the input
        // preprocessor and then emit all of them at once.
        fn emit_virtual_input(&mut self, vi: VirtualInput) {
            // here, we start the cycle for the VFSM execution model. For this simplified
            // state machine implementation, always, at this point, the VFSM will be in the
            // idle state, waiting for VIs to come.
            let current_state = self.states[&self.current_state].clone(); // TODO optimize
                                                                          // that clone

            let condition = Condition {
                and: HashSet::from_iter(vec![vi]),
            };

            // check for input actions

            for (inpus_action_condition, vo) in current_state.input_actions {
                if condition.and.is_subset(&inpus_action_condition.and) {
                    //TODO this may trigger other virtual inputs, need to handle this...
                    // One aspect is: should I gather some virtual outputs from the input
                    // actions and only them execute them or should I execute one by one?
                    // What are the pros and cons of each approach? is it interesting to
                    // have models for both? are there use cases for both approaches or they
                    // are equivalent?
                    //
                    // For this current state machine, VO are not able to emit VI, thus, we
                    // do not need to worry about that right now, but for more complex state
                    // machines, I want to handle this situation.

                    // execute the input action
                    self.emit_virtual_output(vo);
                }
            }

            // check for transition conditions
            for (transition_condition, next_state) in current_state.transitions {
                // TODO continue from here...
                // it is interesting to decouple the execution model from the emission of
                // VI
            }
        }

        // This method will make the interface between the state machine with the IO from
        // function.
        fn consume_char(&mut self, c: char) {
            match c {
                c if c.is_alphanumeric() => self.emit_virtual_input(VirtualInput::ReadAlphanumeric),
                _ => self.emit_virtual_input(VirtualInput::ReadOther),
            };
        }
    }

    pub fn word_counter(text: &str) -> u32 {
        let state_machine = StateMachine::new();

        // process the input into
        12
    }
    */
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
    let mut sm = StateMachine::init();
    for c in text.chars() {
        sm.send_char(c);
    }
    sm.read_counter()
}

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
            }
        };
    }

    test_word_counter!(test_word_counter_reference, word_counter_reference);
    test_word_counter!(test_word_counter_state_machine, word_counter_state_machine);
}
