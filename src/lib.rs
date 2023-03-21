//! # Doomstack
//!
//! Doomstack is a library to easily and efficiently track error propagation in Rust.
//!
//! # Design philosophy
//!
//! When developing complex projects, a single error can propagate through several
//! layers of logic before being handled. Tracking the exact path the error took,
//! while at the same time ensuring code ergonomics and runtime performance, always
//! ranks high on the priorities of an error-handling library.
//!
//! #### Typed vs. non-typed errors
//!
//! Broadly speaking, we can categorize two approaches to Rust error handling. The
//! first is _typed_:
//!
//! ```ignore
//! enum GardeningError {
//!     IrrigationError(IrrigationError),
//!     // ...
//! }
//!
//! enum IrrigationError {
//!     WaterNotFound,
//!     PipeBroken,
//! }
//!
//! impl From<IrrigationError> for GardeningError {
//!     // Wrap `IrrigationError` in `GardeningError::IrrigationError`
//! }
//!
//! fn tend_garden() -> Result<(), GardeningError> {
//!     water_plants()?;
//!     // ...
//! }
//!
//! fn water_plants() -> Result<(), IrrigationError> {
//!     // ...
//! }
//! ```
//!
//! Errors are `struct`s or `enum`s. Depending on convenience or personal preference,
//! one error type is defined per module, structure or function. Typed error handling
//! has the advantage of being structured and exhaustive. For example, all possible
//! causes of an error can be listed in the variants of an `enum`, each possibly
//! nesting another error to enable deeper handling. A fully typed aproach, however,
//! quickly becomes unwieldy, producing large, deeply nested error types that are heavy
//! to allocate on the stack and cumbersome to `match`.
//!
//! On the other end of the spectrum, we have _non-typed_ errors:
//!
//! ```ignore
//! // Often like this..
//! fn water_plants() -> Result<(), Box<dyn std::error::Error>> {
//!     // ...
//! }
//!
//! // .. or sometimes even like this!
//! fn water_plants() -> Result<(), String> {
//!     // ...
//! }
//! ```
//!
//! Non-typed errors are easy to compose and make for simple, uniform method signatures,
//! but they often come at the cost of heap allocations and dynamic dispatch. Most
//! importantly, non-typed errors give up on Rust's powerful file system: the compiler
//! does not know anymore whether or not, for example, all possible causes of an error
//! have been investigated.
//!
//! #### Doomstack's compromise
//!
//! Doomstack tries to offer an healthy balance between typed and non-typed errors:
//! _the most recent error is typed, its predecessors are non-typed_. As in typed error
//! handling, a Doomstack error is a user-defined `struct` or `enum` (implementing the
//! [`Doom`] trait):
//!
//! ```
//! use doomstack::{Description, Doom};
//!
//! struct GardeningError {
//!     tool_involved: Tool,
//!     casualties: u32,
//! }
//!
//! #[derive(Debug)]
//! enum Tool {
//!     Hose,
//!     Rake,
//!     Spade,
//! }
//!
//! impl Doom for GardeningError {
//!     fn tag(&self) -> &'static str {
//!         "GardeningError"
//!     }
//!
//!     fn description(&self) -> Description {
//!         Description::Owned(format!(
//!             "Gardening error involving a {:?}: {} casualties",
//!             self.tool_involved,
//!             self.casualties,
//!         ))
//!     }
//! }
//! ```
//!
//!  To enable non-typed error handling, a `Doom` error can be _archived_ into a common
//!  type, [`Entry`], which captures some of the properties of the original error (e.g.,
//!  [`tag`] and [`description`]) along with (optionally) a `Box<dyn>`-ed copy of the
//!  original error:
//!
//! ```
//! # use doomstack::{Description, Doom};
//! #
//! # struct GardeningError {
//! #     tool_involved: Tool,
//! #     casualties: u32,
//! # }
//! #
//! # #[derive(Debug)]
//! # enum Tool {
//! #     Hose,
//! #     Rake,
//! #     Spade,
//! # }
//! #
//! # impl Doom for GardeningError {
//! #     fn tag(&self) -> &'static str {
//! #         "GardeningError"
//! #     }
//! #
//! #     fn description(&self) -> Description {
//! #         Description::Owned(format!(
//! #             "Gardening error involving a {:?}: {} casualties",
//! #             self.tool_involved,
//! #             self.casualties,
//! #         ))
//! #     }
//! # }
//! #
//! use doomstack::Entry;
//!
//! let error = GardeningError {
//!     tool_involved: Tool::Rake,
//!     casualties: 3,
//! };
//!
//! if error.casualties > 2 {
//!     println!(
//!         "We should probably do something about that deadly {:?} in the garden!",
//!         error.tool_involved,
//!     );
//! }
//!
//! let entry = Entry::archive(error);
//!
//! // `entry` has concrete type `Entry`, exposing some useful methods such as
//! println!("{}", entry.description());
//! // but we can no longer, e.g., get `entry.tool_involved` as a concrete `Tool`.
//! ```
//!
//! (In the example above, the original `error` could be retrieved from `entry` if
//! `GardeningError` prescribed to [`keep_original`], but let's keep our examples
//! as simple as possible for now).

//! Multiple [`Entry`]-ies can be stacked in a [`Stack`], allowing to track error
//! propagation. Because every [`Doom`] error can be converted into an [`Entry`],
//! a [`Stack`] can seamlessly archive errors of different types:
//!
//! ```
//! use doomstack::Stack;
//!
//! fn could_go_wrong() -> Result<(), Stack> {
//!     // This could return, e.g., a `Stack` with an `HoseError`, a
//!     // `GardeningError` and a `LandscapingError` error, each archived
//!     // into an `Entry`.
//!     # Ok(())
//! }
//! ```
//!
//! Between typed [`Doom`]s and non-typed [`Stack`]s, Doomstack offers [`Top`]s.
//! Simply put, a `Top<MyError>` is a (typed) `MyError`, sitting on top of a
//! (non-typed) [`Stack`] of [`Entry`]-ies, archiving `MyError`'s predecessors.
//! `MyError` is the most recent error, and as such, it is typed, stored on the
//! stack, and ready for exhaustive, programmatic handling. `MyError`'s
//! predecessors, whose types are less likely to contribute useful error-handling
//! details, are archived, tracking the sequence of errors that led to `MyError`
//! without bloating stack and type system:
//! ```
//! # use doomstack::{Description, Doom, Stack};
//! #
//! # struct GardeningError {
//! #     tool_involved: Tool,
//! #     casualties: u32,
//! # }
//! #
//! # #[derive(Debug)]
//! # enum Tool {
//! #     Hose,
//! #     Rake,
//! #     Spade,
//! # }
//! #
//! # impl Doom for GardeningError {
//! #     fn tag(&self) -> &'static str {
//! #         "GardeningError"
//! #     }
//! #
//! #     fn description(&self) -> Description {
//! #         Description::Owned(format!(
//! #             "Gardening error involving a {:?}: {} casualties",
//! #             self.tool_involved,
//! #             self.casualties,
//! #         ))
//! #     }
//! # }
//! #
//! # fn could_go_wrong() -> Result<(), Stack> {
//! #   Ok(())
//! # }
//! #
//! use doomstack::Top;
//!
//! fn tend_garden() -> Result<(), Top<GardeningError>> {
//!     if let Err(stack) = could_go_wrong() {
//!         return Err(stack.push(
//!             GardeningError {
//!                 tool_involved: Tool::Rake,
//!                 casualties: 4,
//!             }
//!         ));
//!     }
//!
//!     Ok(())
//! }
//!
//! if let Err(top) = tend_garden() {
//!     println!("This time the casualties were {}", top.doom().casualties);
//!     println!("Leading up to the catastrophe:");
//!     for entry in top.stack().entries() {
//!         println!("{:?}", entry);
//!     }
//! }
//! ```
//!
//! The above examples are lengthy and pedantic to be more understandable, but don't
//! worry: Doomstack comes packed macros and syntax sugar that makes error handling
//! easy as cake. Read around to learn more!
//!
//! [`tag`]: Doom::tag
//! [`description`]: Doom::description
//! [`keep_original`]: Doom::keep_original

mod description;
mod doom;
mod entry;
mod here;
mod location;
mod result_ext;
mod stack;
mod top;

pub use description::Description;
pub use doom::Doom;
pub use entry::Entry;
pub use location::Location;
pub use result_ext::ResultExt;
pub use stack::Stack;
pub use top::Top;
