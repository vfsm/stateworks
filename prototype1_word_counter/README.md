# Prototype 1: Word Counter
In this prototype, a word counter was implemented using the VFSM
technique as describe by the method stateWORKS, with some adaptations
for the execution model.

The focus was completely on the engine of the state machine. In other
words, no effort was directed to create a DSL, parser, etc. At the
moment, we are focusing on building something that works properly for
merging software with state machines.

Also, the main goal of this series of prototypes is to give the
foundation for building a state machine that has the following
characteristics:
- **Synchronicity**: we can solve a lot of interesting problems using only
  synchronous applications, thus, it is not useless to start with an
  synchronous version of the state machine model. Also, asynchronous
  problems are VERY hard. Starting the exploration with this added
  complexity would slow the process significantly.

- **Determinism**: here, we are not using the meaning from classic
  literature of state machines (although, we will use *deterministic*
  state machines in the classical sense also), but, instead,
  deterministic in the sense that, given a set of actions for some state
  machine specification, the result will be always the same. By result,
  interpret the sequence of states for all state machines of the system,
  but not only that, also a sequence of virtual inputs and outputs and
  any other important aspect that can be used to describe the state of
  the system at some point. 
  Related to this aspect, for example, the implementation differs a
  little bit from the original (described in the stateWORKS method). In
  the original execution model, the order of execution of input actions
  is unpredictable. In this execution model, it is predictable.

- **Intuitive**: once designed, the behavior of the state machine must
  be intuitive and easy to make its steps mentally. 

- **Passiveness**: the initial prototypes and even beta models may have
  some degree of passiveness given that this makes the implementation
  simpler. For example, this model works in the following cycle:
    1. User sends data/input to the model through an API which is a method.
    2. The model transforms the data or the input into virtual input.
    3. The model, which was initially in the idle state, now executes
    4. When it comes back to the idle state, the execution finished and
       the flow comes back to the user, and the program is able to
       continue its execution.
  The degree of passiveness will decrease when asynchronous elements or
  models are added. One example, is the addition of a timer to allow
  timeout.  

- **Good Performance**: the implementation will not be optimized for
  performance, but it is very important to take notes and make
  observations about hot spots related to performance optimizations. We
  will start to implement something with good performance during the
  beta version.

- **Isolation**: We want to have a model that is decoupled from the
  remaining of the software. The programmer will interact with it by a
  very well defined API. The complete specification of the state machine
  will be done using tables (state diagram and state table). Only some
  elements of the interface may have to be implemented by the user.
  Every input to the state machine is transformed into VI and every
  output was transformed from a VO.

- **User Experience**: the same way as for performance, the user
  experience will not be rigorously addressed during this stage, but it
  is important to take notes on important spots and be aware to work on
  them during the main implementation.


## Lessons
From this implementation, some lessons will be extracted and described
below. They will be used as aid for the next prototype implementations
and for the implementation of the first beta model.

- Although state machines appear to be simple, there are a lot of decisions to
  make while designing their execution model. There are a lot of unknown
  trade-offs that must be analyzed before each decision.

- Some challenges:
    - How to implement systems of state machines. The current
      implementation only have 1 state machine, but it is desired to
      have a system of them. One of the challenges (and one important
      decision that will be made) is how the execution of the
      state machines should be handled. The next prototype will address
      that.
    - The virtual input originally (stateWORKS original description) is
      only of the static input kind. Basically, when someone want to
      simulate an one-time signal, one has to make a transition followed
      by a timeout to go back to the previous state or engineer the
      state machine in such a way that an action will be taken to go
      back to the previous state. 
      For example: (user presses a button) => command goes from IDLE to 
      PRESSED => a timer is activated => some time later, the state goes
      back to IDLE. 
      Then, during this prototype, we decided to add a 
      native one-time signal, the problem is, how can we handle it? 
      Until now, we decided to make it expire after the first state 
      transition. In terms of user experience, the user would be visually 
      aware of which signal is one-time and which is static.
    - How to add asynchronous elements? Let's think about a non-blocking 
      timer, for example.
    - How to properly add a display/debug representation for the state
      machine. 
    - Is it possible that a VO triggers the emission of a VI? Should I
      worry about that? How would that be handled by the execution
      model?

- A very obvious interface between the state machine and the programming
  world (API) can be a struct (Rust) or a class for object oriented
  languages (C++, Java, JavaScript, etc), or a header with the
  declarations (C). 

- The user will only need to write some parts of the IO interfaces that
  he decides to include in his design. Also, he will be able to interact
  with the state machine programatically by interacting with the
  struct/class instance.

- One-time signal was added to this prototype because it appears to
  simplify some use cases, specially for a programming context. But that
  is something that must be analyzed better. What are the pros and cons?
  What are the alternatives?

- One-time signals will be consumed after, at most, the first state 
  transition (we may think and investigate the future possibility of 
  one-time signals that last multiple state transitions, but that may 
  not be something useful), and if no state transition was done, they 
  will be consumed when the execution returns to idle state. Static 
  signals will last until they are changed.

- One-time signals and static signals are equivalent in terms of
  positive logic for VI, but they may be separated for performance
  reasons. Thus, this may be a performance hot spot.

- one-time signals will be consumed only once after they are emitted. Once
  consumed (even if they did not trigger anything), this kind of signal become absent
  until a new one comes. Their absence cannot be used as signal. As an example, we have
  a programming function that transforms a programming input into an VI. The
  programming function may generate any number of families of one-time VI signals.


- I think one-time signals and static signals can be considered as
  events and conditions respectively. This way we can understand better
  their difference. 

- Transition actions were included because they simplify some use cases 
also. But it is also important to analyze the pros and cons.


- IO objects used:
    - SyncInputStream: this IO object can generate events based on the
      input it receives. In the prototype, it generates events like:
      AlphanumericRead when the input is an alphanumeric char.
    - Variable: this IO object allows the state machine to have a
      variable that may have interface between the variable and the
      external world (read/write) and between the variable and the state
      machine (read as VI [Virtual Input] and write as VO[Virtual Output]). 
      The user chose only read for IO/external and write via VO for
      IO/state machine. He will be able to define the name of the
      methods of the API and part of the code.

- The state machine is automatically executed once when it is created.
  That allows the user to design it in such a way that any
  initialization can be made at this state without the necessity of any
  event.

- Would it be interesting to provide a function wrapper functionality
  for the state machine in order to allow an even more rigid decoupling?


