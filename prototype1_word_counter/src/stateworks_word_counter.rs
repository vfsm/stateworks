use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::OnceLock,
};

// user implemented functions...
fn print_hello() {
    println!("Hello world from state machine");
}

//---------------------------------------------------------------------------------------

// Virtual Inputs: They may be either static signal or events (one-time signal)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Event {
    // IO-Object: CharStreamInput
    ReadAlphanumeric,
    ReadOther,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum StaticSignal {
    // Always static signal
    Always,
}

#[derive(Debug, Clone, Copy)]
enum VirtualOutput {
    PrintHello,

    // IO-Object: Counter
    IncrementCounter,
}

#[derive(Debug)]
struct VirtualInput {
    events: HashSet<Event>,
    static_signals: HashSet<StaticSignal>,
}

impl VirtualInput {
    fn new() -> Self {
        Self {
            events: HashSet::new(),
            static_signals: HashSet::new(),
        }
    }
    fn is_subset(&self, other: &VirtualInput) -> bool {
        self.events.is_subset(&other.events) && self.static_signals.is_subset(&other.static_signals)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Init,
    InWord,
    OutWord,
}

#[derive(Debug)]
struct StateSpec {
    name: &'static str,
    entry_actions: Vec<VirtualOutput>,
    exit_actions: Vec<VirtualOutput>,
    /// (condition, actions)
    input_actions: Vec<(VirtualInput, VirtualOutput)>,
    /// (condition, state, actions)
    transitions: Vec<(VirtualInput, State, Vec<VirtualOutput>)>,
}

#[derive(Debug)]
pub struct StateMachine {
    current_state: State,
    virtual_input: VirtualInput,
    // TODO maybe creating a StateMachine struct as the top level is not the best approach. Create an
    // RTDB/VFSM struct may be better. Every IO object and state machine may be included
    // to this VFSM. I don't know, something like that.
    counter: u32,
}
static STATES: OnceLock<HashMap<State, StateSpec>> = OnceLock::new();
static GLOBAL_INPUT_ACTIONS: OnceLock<Vec<(VirtualInput, VirtualOutput)>> = OnceLock::new();

fn get_global_input_actions() -> &'static Vec<(VirtualInput, VirtualOutput)> {
    &GLOBAL_INPUT_ACTIONS.get_or_init(|| {
        vec![(
            VirtualInput {
                events: HashSet::new(),
                static_signals: HashSet::from_iter(vec![StaticSignal::Always]),
            },
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
                        VirtualInput {
                            events: HashSet::new(),
                            static_signals: HashSet::from_iter(vec![StaticSignal::Always]),
                        },
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
                        VirtualInput {
                            events: HashSet::from_iter(vec![Event::ReadAlphanumeric]),
                            static_signals: HashSet::new(),
                        },
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
                        VirtualInput {
                            events: HashSet::from_iter(vec![Event::ReadOther]),
                            static_signals: HashSet::new(),
                        },
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
            State::InWord => "InWord",
            State::OutWord => "OutWord",
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
            virtual_input: VirtualInput::new(),
            current_state: State::Init,
            counter: 0,
        };
        // initiate the state machine
        state_machine.execute();
        state_machine
    }

    fn accepts_condition(&self, condition: &VirtualInput) -> bool {
        condition.static_signals.contains(&StaticSignal::Always)
            || condition.is_subset(&self.virtual_input)
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
        self.virtual_input.events.clear();
    }

    fn emit_events(&mut self, events: Vec<Event>) {
        self.virtual_input.events.extend(events);
    }

    // StreamInputInterface
    // (this function will be implemented and designed, including name, by the user)
    pub fn send_char(&mut self, c: char) {
        println!("#########################################");
        println!("#==> CharInputInterface emitting: '{c}' #");
        println!("#########################################");
        // TODO the user must implement only the function that defines which VI will be
        // triggered. This method (send_char) must not be edited by the user.
        // user_callback(c);
        if c.is_alphanumeric() {
            self.emit_events(vec![Event::ReadAlphanumeric]);
        } else {
            self.emit_events(vec![Event::ReadOther]);
        }
        // finish user callback

        // execute only after emitting an event
        self.execute();
    }

    // CounterOutputInterface
    pub fn read_counter(&self) -> u32 {
        // TODO here, the same, the user must be allowed only to edit a callback that
        // will receive the variable as input (allowing logs, transformations, etc), but
        // nothing more.
        println!("(counter = {})", self.counter);
        self.counter
    }

    fn increment_counter(&mut self) {
        // TODO the user must only implement a callback which receive the reference to
        // the variable that will be changed
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
