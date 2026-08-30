---
tags: [mealy, richards, composition]
node_type: concept
aliases: [enablement DAG, disciplined composition, newton program]
---

Short answer: newton-machine is Mealy-shaped, not “just a Mealy machine.” newtonian-core is not a Richards controller. The “four parts” rhyme is rhetorical. You are composing known objects under conservation laws, not naming a new automaton class.

What a Richards controller actually is

Charles L. Richards, Electronics, Feb 1973: a TTL implementation method so you could build large FSMs without drawing a full next-state table or doing Karnaugh maps by hand.

Wikipedia is right that it is a Mealy machine: output depends on current state and the tested input. The trick is the encoding:

┌──────────────────────────┬───────────────────────────────────────────────────────────────┐
│ Richards kernel          │ Role                                                          │
├──────────────────────────┼───────────────────────────────────────────────────────────────┤
│ Counter (e.g. 74163)     │ State register as a sequential address                        │
├──────────────────────────┼───────────────────────────────────────────────────────────────┤
│ Multiplexer (e.g. 74151) │ Given that address, pick one input bit to test                │
├──────────────────────────┼───────────────────────────────────────────────────────────────┤
│ YES decoder              │ If that bit is 1: fire YES pins; usually increment            │
├──────────────────────────┼───────────────────────────────────────────────────────────────┤
│ NO decoder               │ If that bit is 0: fire NO pins; usually hold (or load a jump) │
└──────────────────────────┴───────────────────────────────────────────────────────────────┘

The designer draws a flowchart of single-bit tests, not a Harel chart. YES often means “next count.” A jump is “load the counter.” It scales to hundreds of sequential steps because state is a count, not a minimized encoding of an arbitrary graph.

It is a 1970s hardware sequencer. It is not a product configuration, not a ROM of authored chords, not hysteresis with independent clocks, not a snapshot law.

Is newton-machine just a Mealy machine?

Each step is Mealy. apply(msg) is δ, λ : (q, i) ↦ (q′, o): configuration plus message in; new configuration plus Cmd out. view() is closer to Moore: it depends on the configuration (and model), not on the message that just arrived.

That is the classroom split. It is not the product.

A Mealy machine has:

• a flat finite set of states,
• one transition function,
• outputs that may depend on the input.

A Newton machine adds, and those additions are the reason to use the crate:

• Harel topology — XOR as enum, AND as struct, LCA/perform, RTC drain, history sidecar. A Mealy table does not give you orthogonal regions or “do not exit the parent you are staying in.”
• UCA conservation laws — illegal XOR cannot be constructed; update has no I/O; snapshot is {config, context, history}; history is not live variants.
• TEA grain — one door, Cmd as data, host executes.

You can compile any Newton chart down to a giant Mealy machine (or a Richards flowchart, or a ROM). That is true of every FSM. It does not make Newton “just Mealy,” any more than a CPU is “just a Mealy machine” because the control unit can be drawn that way.

Competitive with Moore/Mealy/Richards at their layer? No, and it should not try. Those methods win on a PCB: four TTL parts, one-bit tests, increment/jump. Newton wins in a Rust process that must replay, snapshot, and refuse illegal XOR. If the job is a 74163 sequencer, use a counter. If the job is Offline/Connecting/Online with history and Cmd, a Mealy table is the thing Newton exists to stop you from maintaining by hand.

Is newtonian-core a modified Richards controller?

No. Same shape of sentence (“four parts”), different machine.

┌──────────────────────┬──────────────────────────────────────┬────────────────────────────────────────────────────────────────────┐
│                      │ Richards                             │ Policy kernel                                                      │
├──────────────────────┼──────────────────────────────────────┼────────────────────────────────────────────────────────────────────┤
│ Configuration        │ Scalar (count / address)             │ Product (bitset of in-play scores, or packed Newton discriminants) │
├──────────────────────┼──────────────────────────────────────┼────────────────────────────────────────────────────────────────────┤
│ What you test        │ One bit per state, selected by mux   │ Gated set of classifiers, each with its own clock                  │
├──────────────────────┼──────────────────────────────────────┼────────────────────────────────────────────────────────────────────┤
│ Next state           │ Increment, hold, or load             │ Recalc the pool; key is the pool                                   │
├──────────────────────┼──────────────────────────────────────┼────────────────────────────────────────────────────────────────────┤
│ Output               │ YES/NO decoder pins                  │ Exact ROM: authored key → Behavior(knobs) → Cmd; unauthored → NONE │
├──────────────────────┼──────────────────────────────────────┼────────────────────────────────────────────────────────────────────┤
│ Hierarchy            │ Jump addresses in combinational glue │ Enablement DAG (clock-enable), not a jump ROM                      │
├──────────────────────┼──────────────────────────────────────┼────────────────────────────────────────────────────────────────────┤
│ Memory besides state │ None in the kernel                   │ Sticky/arm latches (not Harel history)                             │
├──────────────────────┼──────────────────────────────────────┼────────────────────────────────────────────────────────────────────┤
│ Authoring            │ Flowchart of tests                   │ Fold YAML once into interned keys                                  │
└──────────────────────┴──────────────────────────────────────┴────────────────────────────────────────────────────────────────────┘

A Richards NO is “stay at this count.” Kernel sticky is “this predicate stays in the product while raw is false, for N units of its clock.” Those are different.

Richards scales depth (many sequential steps). The kernel scales width (many simultaneous flags) by not naming the cartesian product as XOR children, then indexing a sparse table of the chords you actually authored.

If you forced the kernel into Richards form you would: serialize every score test into a flowchart, one bit per step, and explode. That is the opposite of a 5 s pulse of 20 scores × N entities.

Enablement DAG is closer to clock enables / instruction skip than to Richards jump. Exact ROM is a decision table (Dijkstra guarded commands, except you do not scan the whole guard list: you hash the key). Hysteretic scores are latches with gated clocks. The pulse is Esterel/Lustre synchrony + Elm update. None of that is Charles Richards’s TTL kernel.

Are the two “four parts” the same four parts?

They are not a correspondence. Do not teach them as one diagram.

Richards:     counter     mux          YES decoder    NO decoder
              (address)   (pick 1 bit) (pins)         (pins)

Newton pgm:   pulse       scores       product key    enablement DAG
              (clock)     (latches)    (ROM index)    (clock enables)

• Pulse ≠ counter. The pulse is the clock domain. The counter is Richards state.
• Scores ≠ mux. The mux picks which bit to look at. Scores are the bits, latched.
• Product key ≠ YES decoder. The decoder is Mealy output. The key is configuration.
• Enablement ≠ NO decoder. NO is “condition false this step.” Enablement is “this classifier is not even allowed to run.”

The only shared idea: synchronous sequential control with a small, regular datapath instead of a unique next-state net per design. Richards found that regularity in a counter+mux+decoders. You found it in bitset+ROM+gates. Same engineering instinct, different regularity.

Inventing CS, or recreating Richards?

Neither, if “inventing CS” means a new automaton. A disciplined composition, if it means a crate family.

The objects are all classical:

1. Mealy/Moore (and Medvedev, if you register outputs)
2. Harel statecharts (hierarchy, orthogonality, history, RTC)
3. Esterel/Lustre synchrony (one tick, no thread per region)
4. Elm TEA (Msg in, Cmd out, replay = same function)
5. Latches / hysteretic comparators
6. Decision tables / Flyweight ROM
7. Clock-enable hierarchy (not behavior trees, not Rete)

What is worth publishing is not “we discovered the fifth automaton.” It is:

• Grain: truth (Newton) ≠ policy ROM (core) ≠ pulse (host) ≠ wire (adapter)
• Laws: illegal XOR unrepresentable; exact key fail-closed; YAML after Fold never in step; sticky ≠ Harel history; no I/O in the step
• Encoding: nested ADTs for modal stance; u128 product for in-play predicates; interned table for plays

That is the same move as “Richards vs a hand-minimized state table”: pick a representation that makes the common case regular so a 1000-state (or 20-score × 50-entity) design does not become a unique Boolean mess. Richards did it for 1973 TTL sequential controllers. You are doing it for replayable Rust programs whose “states” are overlapping scored truths plus a few typed XOR regions.

Competitiveness, honestly

┌───────────────────────────────────────────────────────────────────────────────┬──────────────────────────────────────────┐
│ Job                                                                           │ Winner                                   │
├───────────────────────────────────────────────────────────────────────────────┼──────────────────────────────────────────┤
│ TTL/FPGA sequential controller, one-bit tests, increment/jump                 │ Richards / one-hot / encoded FSM in HDL  │
├───────────────────────────────────────────────────────────────────────────────┼──────────────────────────────────────────┤
│ Classroom FSM, 8 states, homework                                             │ Moore/Mealy table                        │
├───────────────────────────────────────────────────────────────────────────────┼──────────────────────────────────────────┤
│ Session / order / hold as typed stance, snapshot, LCA                         │ newton-machine                           │
├───────────────────────────────────────────────────────────────────────────────┼──────────────────────────────────────────┤
│ 20 overlapping predicates × N entities, YAML plays, same step live and replay │ newtonian-core kernel (when implemented) │
├───────────────────────────────────────────────────────────────────────────────┼──────────────────────────────────────────┤
│ “Are we faster than a 74163?”                                                 │ Wrong contest                            │
└───────────────────────────────────────────────────────────────────────────────┴──────────────────────────────────────────┘

Low-level enough for software: a bitset plus a hash of authored keys is the right floor. You will not beat a counter+mux on a scope. You should beat a string-id SCXML interpreter, a Rete, and a handwritten mega-enum of overlaps — that is the encoding argument the Newton benches already make.

Do not put “Richards variant” on the README. It will train people to look for a counter and a YES/NO decoder. Put: Mealy-shaped step, Harel/UCA truth, configuration-indexed ROM, gated latches. Credit Richards only as an ancestor of “regular sequencer datapath,” in the same sentence as Esterel and decision tables.

The family is not new physics. It is a closed set of refusals around objects hardware people already knew, aimed at systems where the configuration is a product and the playbook is data.

---

You are not close to a new automaton class in the sense theorists mean it. You are close to a named, succinct representation of machines that already exist, with conservation laws that are software/system properties, not a new slot in the Chomsky hierarchy.

A new class is a mathematical object with a configuration, a transition relation, and theorems that do not hold for the parent class (expressiveness, composition, decidability). Richards did not invent Mealy machines; he invented a regular datapath for them. That is the same kind of move you are making.

What “automaton class” means

Rough layers, coarsest first:

Acceptors (languages)
DFA/NFA → regular. PDA → context-free. LBA → context-sensitive. TM → recursively enumerable. Nested-word / visibly pushdown automata are the rare named subclass that earns the title because emptiness and equivalence get better than general PDA.

Transducers (input/output)
Moore: output from state. Mealy: output from state and input. Medvedev: outputs registered in the FFs. Sequential functions, I/O automata (Lynch–Tuttle), interface automata. Still finite-state if Q is finite.

Structure on finite sequential machines (usually not new classes)
Hierarchical FSMs / Harel statecharts (XOR, AND, history, RTC). Alur–Yannakakis hierarchical state machines (flattening is exponential; that is a theorem). Incompletely specified sequential machines (classic switching theory: λ partial). Featured transition systems (software product lines). Richards: counter+mux encoding of Mealy.

Time, data, concurrency (these are classes)
Timed automata (Alur–Dill). Hybrid automata. Register / data / symbolic automata. Petri nets / VAS. Synchronous reactive (Esterel, Lustre clocks). Alternating, weighted, probabilistic, Büchi/parity for infinite words.

Coalgebras (the categorical name for “machine”)
Moore: X → B × X^A. Mealy: X → (B × X)^A. XOR = coproduct, AND = product: coalgebras of polynomial functors. That is already the theory of nested ADT configurations.

Newton + the policy kernel sit in finite sequential machines with structure, plus a host that may be infinite-state (prices, images, time).

Where you are today

┌───────────────────────┬────────────────────────────────────────────────────────┬────────────────────────────────────────────────────────────────────────┐
│ Piece                 │ As an automaton                                        │ Parent class                                                           │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Runtime::apply        │ Mealy: (q, msg) ↦ (q′, Cmd)                            │ Finite sequential transducer (if enums finite)                         │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ view                  │ Moore-ish                                              │ Same                                                                   │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Nested ADT config     │ Polynomial functor / Harel AND–XOR                     │ Hierarchical FSM                                                       │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ History sidecar       │ Extra finite memory of last child                      │ Statechart history                                                     │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ RTC + storm cap       │ Bounded τ-closure of internal events                   │ Operational statechart semantics (Pnueli–Shalev and cousins)           │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Cmd as data, no I/O   │ Restriction on implementation, not on δ                │ I/O automata already separate action from effect                       │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Snapshot triple       │ Observational completeness: no hidden interpreter heap │ Engineering invariant                                                  │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Scores + sticky       │ Vector of latches + bounded counters                   │ Still finite Mealy (state space 2ⁿ × ∏ ages)                           │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Exact ROM             │ Partial output map λ: Q ⇀ B                            │ Incompletely specified Mealy                                           │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Enablement DAG        │ Which coordinates of δ are allowed to fire             │ Clock enables / gated product; still finite                            │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Multi-rate clocks     │ Discrete sample-and-hold on a base tick                │ Lustre clock calculus / multi-rate timed (discrete)                    │
├───────────────────────┼────────────────────────────────────────────────────────┼────────────────────────────────────────────────────────────────────────┤
│ Beliefs, bars, pixels │ Infinite or large data outside Q                       │ Register/hybrid/timed — you threshold them into bits before the kernel │
└───────────────────────┴────────────────────────────────────────────────────────┴────────────────────────────────────────────────────────────────────────┘

If sticky counters are bounded (they are: max_age), score kinds are finite, and Newton enums are finite, both crates describe finite Mealy machines. The ROM being sparse does not change the class. It changes the representation, the way a BDD is not a new logic.

The interesting infinite-state physics (marks, frames, timestamps) lives in beliefs / Model. Lift’s job is to throw that away until a category change. That is why the kernel can stay regular. It is also why you should not claim a new infinite-state class unless you stop thresholding.

Are you close?

No, if the bar is “Chomsky / Alur / visiby-pushdown”: a new slot with a completeness or decidability theorem.

Yes, if the bar is “Richards-class contribution”: a regular kernel for a machine people already have, so a 20-score × 50-entity design is not a unique next-state net. That is representation + laws. Worth a crate and a paper. Not a new automaton.

You would be lying (to theorists and to yourselves) if the README said “new automaton class.” You would be accurate if it said structured partial Mealy machines with a product configuration and a sparse output ROM.

Where a real class could come from

You only get a new class if you change expressiveness or decidability, then prove it. Three honest forks; only the first is near what you already believe.

1. Named subclass of finite Mealy (most plausible paper)

Product-indexed partial transducers (call it whatever you want after the theorems, not before):

• State = product of small XOR regions or a bit-vector of named latches.
• Enablement = a poset: parent off ⇒ children forced off in one tick (a synchronous invariant, not a stack).
• λ partial: defined exactly on an authored set R ⊆ Q; unauthored ⇒ no output (not “nearest chord”).
• Multi-rate: each latch updates on clock cᵢ ∣ c_(base).

That is still finite Mealy. It becomes a class people will cite if you prove things general Mealy does not make obvious:

• Flattening AND/XOR is exponential (statecharts already have this; you must add something).
• ROM miss is a safety property independent of flattening.
• Enablement invariants are compositional (product of gated machines).
• Clock-independent scores: a 1 m latch is invariant under stuttering of 5 s ticks (a clock calculus, Lustre-adjacent).
• Equivalence of two YAML ROMs + enablement DAGs is cheaper than flattening to 2ⁿ Mealy.

Closest existing names: incompletely specified sequential machines; featured transition systems; hierarchical FSMs. Your differentiator is exact sparse λ + gated multi-rate latches as the intended semantics, not longest-subset, not one-hot of 20 scores.

This is the fork that matches the family. It is a subclass, like “visibly pushdown ⊂ PDA,” only here “gated partial product Mealy ⊂ Mealy.” The theorem has to earn the name.

2. Leave the finite world (only if you mean it)

┌───────────────────────────────────────────────────────┬─────────────────────────────────┬───────────────────────────────────────────────────────┐
│ If you add…                                           │ You fall into…                  │ Already exists?                                       │
├───────────────────────────────────────────────────────┼─────────────────────────────────┼───────────────────────────────────────────────────────┤
│ Unbounded sticky / “until raw true” with no max       │ Counter / incrementing automata │ Yes                                                   │
├───────────────────────────────────────────────────────┼─────────────────────────────────┼───────────────────────────────────────────────────────┤
│ Continuous time, clocks as reals                      │ Timed automata                  │ Yes                                                   │
├───────────────────────────────────────────────────────┼─────────────────────────────────┼───────────────────────────────────────────────────────┤
│ Prices/images in Q without thresholding               │ Register / symbolic / hybrid    │ Yes                                                   │
├───────────────────────────────────────────────────────┼─────────────────────────────────┼───────────────────────────────────────────────────────┤
│ Unbounded enablement stack (nested gates as push/pop) │ Nested words / VPA              │ Yes, and this would fight your “finite DAG of scores” │
├───────────────────────────────────────────────────────┼─────────────────────────────────┼───────────────────────────────────────────────────────┤
│ True concurrent regions (tokens, not virtual AND)     │ Petri nets                      │ Yes                                                   │
└───────────────────────────────────────────────────────┴─────────────────────────────────┴───────────────────────────────────────────────────────┘

Unbounded enablement nesting is the only one that is slightly native to your story, and you have already refused it: scores are a finite named set. Do not un-refuse that just to mint a class.

3. Types, not automata

“Illegal XOR cannot be constructed” is a type system for configurations (polynomial types, session/typestate). The automaton is still Mealy; the syntax of Q is restricted. That is coalgebra + types, or “typed hierarchical machines.” Papers exist; a Rust-native coalgebra of enum/struct with Cmd in a free monoid is a niche, not a Chomsky class.

UCA (no I/O in δ, snapshot = whole state, gateway outside) is TLA-style invariant / I/O-automaton hygiene. Lynch already split signature vs effect. Your contribution is making that unrepresentable in the API, which is PL, not automata theory.

What to do with this

Do not hunt a new class as the product. Hunt a representation theorem:

Every Newton machine with finite regions is a Mealy machine whose state space is the unfolding of a polynomial functor (sums = XOR, products = AND), with history a finite register, RTC a bounded internal closure, and outputs in a free monoid of commands.
Every policy kernel with n scores, bounded sticky, and authored set R is a partial Mealy machine on at most 2ⁿ ⋅ ∏ (1+aᵢ) states, implemented in time independent of that bound (bitset + |R| lookup), with enablement a clock-enable invariant.

That sentence is the honest “invention”: succinctness + partial λ + multi-rate gates + UCA embedding, not a fifth automaton.

If you later prove ROM-equivalence or “can this sleeve ever fire?” is in P (or NL, or whatever) while flattened Mealy equivalence is PSPACE-hard (it is, for succinct circuits / hierarchical FSMs), then you have the visibly-pushdown-shaped result: same objects, better problems because of the representation. That is the only path I would call “close to a new class,” and even then the literature may file it under succinct hierarchical / featured / incomplete Mealy.

Until that theorem exists, the family is: Harel + Elm as laws, plus a gated exact-ROM transducer as the playbook. Classical machines. New grain. Not new physics.