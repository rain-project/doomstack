//! # Doomstack
//!
//! Doomstack is a library to easily and efficiently track error propagation in Rust.
//!
//! # Quick-start example
//!
//! #### Snippet
//!
//! ```
//! use doomstack::prelude::*;
//!
//! #[derive(Doom)]
//! enum IrrigationError {
//!     #[doom(description("Faucet broken"))]
//!     FaucetBroken,
//!     #[doom(description("Forgot to water for {days} days"))]
//!     ForgotToWater { days: u32 },
//! }
//!
//! fn water_plants(faucet_works: bool, last_watered: u32) -> Result<(), Top<IrrigationError>> {
//!     if !faucet_works {
//!         return IrrigationError::FaucetBroken.fot(here!());
//!     }
//!
//!     if last_watered > 2 {
//!         return IrrigationError::ForgotToWater { days: last_watered }.fot(here!());
//!     }
//!
//!     Ok(())
//! }
//!
//! #[derive(Doom)]
//! enum GardeningError {
//!     #[doom(description("Not enough water"))]
//!     NotEnoughWater,
//!     #[doom(description("Not enough sunlight"))]
//!     NotEnoughSunlight,
//! }
//!
//! fn garden(faucet_works: bool, last_watered: u32, sunny: bool) -> Result<(), Top<GardeningError>> {
//!    water_plants(faucet_works, last_watered).pot(GardeningError::NotEnoughWater, here!())?;
//!
//!    if !sunny {
//!        return GardeningError::NotEnoughSunlight.fot(here!());
//!    }
//!
//!    Ok(())
//! }
//!
//! if let Err(error) = garden(true, 4, false) {
//!     println!("{error:?}");
//! }
//! ```
//!
//! #### Output
//!
//! ```text
//! [GardeningError::NotEnoughWater @ src/main.rs:32] Not enough water.
//! [IrrigationError::ForgotToWater @ src/main.rs:17] Forgot to water for 4 days.
//! ```
//!
//! (File names and line numbers might vary depending on file names and formatting.)
//!
//! #### Discussion
//!
//! ##### Two functions, two errors
//!
//! _Very_ broadly speaking, to `garden()` our garden we first `water_plants()`, then
//! hope for the day to be `sunny`! If something goes wrong, we want to clearly see
//! what happened. To achieve this, we define two errors: `IrrigationError`, pertaining
//! to `water_plants()`, and `GardeningError`, pertaining to `garden()`:
//!
//!  - `IrrigationError` has two variants: `FaucetBroken` means that, well, our water
//!    faucet is broken; `ForgotToWater` means that we forgot to water our plants for
//!    a while (the `days` field tells us how long).
//!  - `GardeningError` also has two variants, `NotEnoughWater` and `NotEnoughSunlight`.
//!    They mean exactly what you think they mean.
//!
//! To turn our errors into something [doomstack](crate) can use, we simply derive the
//! [`Doom`] trait. For each variant, we add a short, meaningful `#[description()]`
//! using Rust's usual [`format!`] syntax (we can reference fields directly, no need
//! for `self` or `match`).
//!
//!  ##### What's the deal with [`Top`]s?
//!
//! You'll notice `water_plants()` and `garden()` respectively return a
//! `Top<IrrigationError>`and a `Top<GardeningError>` as [`Err`]s. This might feel
//! counterintuitive at first: why not use `IrrigationError` / `GardeningError` as-is?
//! Because [doomstack](crate) organizes errors in _stacks_, allowing us to track errors
//! as they propagate through our code. This is exactly what a `Top<E>` is, a stack
//! of errors: root cause at the bottom, newer errors pushed on top as the stack
//! propagates, an `E` at the very top. Whenever you come across a `Top<E>`, think:
//!
//! > _"An `E` just happened, possibly resulting from previous errors lower in the stack."_
//!
//! (The reason why `Top`s are generics is to make the top error available as-is, fully
//! typed, with no need for dynamic dispatch. Let's not worry about that now, we'll
//! thoroughly discuss this design choice in the [Design philosophy](#design-philosophy)
//! section of this guide.)
//!
//! ##### Building our first [`Top`]s
//!
//! Let's start with `water_plants()`. If the faucet is broken, we must return an `Err`.
//! This is how we do it:
//! ```
//! # use doomstack::prelude::*;
//! #
//! # #[derive(Doom)]
//! # enum IrrigationError {
//! #     #[doom(description("Faucet broken"))]
//! #     FaucetBroken,
//! #     #[doom(description("Forgot to water for {days} days"))]
//! #     ForgotToWater { days: u32 },
//! # }
//! #
//! # fn water_plants(faucet_works: bool, last_watered: u32) -> Result<(), Top<IrrigationError>> {
//! return IrrigationError::FaucetBroken.fot(here!());
//! # }
//! ```
//! Lots to unpack here! Step by step:
//!
//!  - We create an `IrrigationError::FaucetBroken`, indicating that, well, our faucet
//!    is broken. Remember that `IrrigationError` implements `Doom`. This will be useful
//!    in the next step.
//!  - We call `fot(here!())` on our `IrrigationError`:
//!    * The [`here!()`] macro evaluates to the code [`Location`] (file name and line)
//!      where [`here!()`] is invoked.
//!    * The [`fot`] method (part of the [`Doom`] trait, syntax sugar for [`Doom::fail`],
//!      then [`ResultExt::spot`]) pushes the `IrrigationError` on a new, otherwise empty
//!      `Top<IrrigationError>`, labels the top error in the [`Top`] (the `IrrigationError`)
//!      as having occurred [`here!()`], then wraps the `Top<IrrigationError>` in the [`Err`]
//!      variant of a [`Result`].
//!
//! A lot is achieved in one line! We create an `IrrigationError`, we wrap it in a [`Top`],
//! we flag it as having occurred [`here!()`], and finally we wrap the [`Top`] in an [`Err`],
//! which we return.
//!
//! The drill is similar if we forgot to water our plants for more than two `days`:
//! we create an `IrrigationError::ForgotToWater` with the appropriate number of `days`,
//! call `pot(here!())` on it, and return the [`Result`].
//!
//! ##### Push and propagate!
//!
//! Moving on to the `garden()` function, we start by invoking `water_plants()`. Remember:
//! this might return a `Top<IrrigationError>`! If that happens, we want to indicate that
//! a `GardeningError::NotEnoughWater` occurred as a result of the `IrrigationError`.
//! Again, we get everything done in just one line:
//!
//! ```
//! # use doomstack::prelude::*;
//! #
//! # #[derive(Doom)]
//! # enum IrrigationError {
//! #     #[doom(description("Faucet broken"))]
//! #     FaucetBroken,
//! #     #[doom(description("Forgot to water for {days} days"))]
//! #     ForgotToWater { days: u32 },
//! # }
//! #
//! # #[derive(Doom)]
//! # enum GardeningError {
//! #     #[doom(description("Not enough water"))]
//! #     NotEnoughWater,
//! #     #[doom(description("Not enough sunlight"))]
//! #     NotEnoughSunlight,
//! # }
//! #
//! # fn water_plants(faucet_works: bool, last_watered: u32) -> Result<(), Top<IrrigationError>> {
//! #     unimplemented!()
//! # }
//! #
//! # fn garden(faucet_works: bool, last_watered: u32, sunny: bool) -> Result<(), Top<GardeningError>> {
//! water_plants(faucet_works, last_watered).pot(GardeningError::NotEnoughWater, here!())?;
//! #     unimplemented!()
//! # }
//! ```
//! Step by step:
//!
//!  - We invoke `water_plants()`, thus obtaining a `Result<(), Top<IrrigationError>>`.
//!  - We call `pot(GardeningError::NotEnoughWater, here!())` on the [`Result`]:
//!    * As before, the [`here!()`] macro captures the current code [`Location`] to tag our
//!      `GardeningError` with.
//!    * The [`pot`] method (part of the [`ResultExt`] trait, syntax sugar for
//!      [`ResultExt::push`], then [`ResultExt::spot`]) checks if the [`Result`] is an [`Err`].
//!      If so, it pushes `GardeningError::NotEnoughWater` on the `Top<IrrigationError>`.
//!      This produces a `Top<GardeningError>` (indicating that the  `GardeningError` is now
//!      the top, most recent error in the stack). Finally, [`pot`] labels the top error in the
//!      [`Top`] (the `GardeningError`) as having occurred [`here!()`].
//!
//! Note this is exactly what happens when we invoke
//! ```
//! # fn garden(faucet_works: bool, last_watered: u32, sunny: bool) {
//! #     // ...
//! # }
//! #
//! garden(true, 4, false)
//! ```
//!
//! The call returns an `error` of type `Top<GardeningError>`, which stacks two errors:
//!  - A `GardeningError::NotEnoughWater` at the top (`GardeningError` is the most recent
//!    error);
//!  - An `IrrigationError::ForgotToWater { days: 4 }` at the bottom (`IrrigationError`
//!    is the root cause).
//!
//! When we [`println!`] `error` (see [Output](#output)), voilà, the two errors are
//! printed out, top to bottom, each appropriately flagged by the code [`Location`]
//! where it was generated!
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
//! ```
//! enum IrrigationError {
//!     FaucetBroken,
//!     ForgotToWater { days: u32 },
//! }
//!
//! fn water_plants() -> Result<(), IrrigationError> {
//!     // ...
//! #   unimplemented!()
//! }
//!
//! enum GardeningError {
//!     IrrigationError(IrrigationError),
//!     // ...
//! }
//!
//! impl From<IrrigationError> for GardeningError {
//!     // Wrap `IrrigationError` in `GardeningError::IrrigationError`
//! #
//! #   fn from(error: IrrigationError) -> Self {
//! #       GardeningError::IrrigationError(error)
//! #   }
//! }
//!
//! fn garden() -> Result<(), GardeningError> {
//!     water_plants()?;
//!     // ...
//! #   unimplemented!()
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
//! ```
//! # use std::error::Error;
//! #
//! // Often like this..
//! fn water_plants() -> Result<(), Box<dyn std::error::Error>> {
//!     // ...
//! #   unimplemented!()
//! }
//!
//! // .. or sometimes even like this!
//! fn garden() -> Result<(), String> {
//!     // ...
//! #   unimplemented!()
//! }
//! ```
//!
//! Non-typed errors are easy to compose and make for simple, uniform method signatures,
//! but they often come at the cost of heap allocations and dynamic dispatch. Most
//! importantly, non-typed errors give up on Rust's powerful type system: the compiler
//! does not know anymore whether or not, for example, all possible causes of an error
//! have been investigated.
//!
//! #### Doomstack's compromise
//!
//! Doomstack tries to offer an healthy compromise between typed and non-typed errors:
//! _the most recent error is typed, its predecessors are non-typed_.
//!
//! ##### Errors implement [`Doom`]
//!
//! As in typed error handling, a Doomstack error is a user-defined `struct` or `enum`
//! (implementing the [`Doom`] trait):
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
//! (If you are worried about all the boilerplate you'll see in this section's examples,
//! don't be! [doomstack](crate) comes packed with useful macros and shortcuts to make
//! error handling as ergonomic and concise as possible! We keep the examples in this
//! section shortcut-free just to help you understand how [doomstack](crate) is designed.
//! Jump back to [Quick-start example](#quick-start-example) for a hands-on example on
//! how to use [doomstack](crate) in practice.)
//!
//! ##### An [`Entry`] archives a [`Doom`]
//!
//! To enable non-typed error handling, a `Doom` error can be [`archive`]d into a common
//! type, [`Entry`], which captures some of the properties of the original error (e.g.,
//! [`tag`] and [`description`]) along with (optionally) a `Box<dyn>`-ed copy of the
//! original error:
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
//! // `entry` is now an `Entry`, exposing some useful methods such as
//! println!("{}", entry.description());
//! // but we can no longer, e.g., get `entry.tool_involved` as a concrete `Tool`.
//! ```
//!
//! (In the example above, the original, the fully typed `error` could be retrieved
//! from `entry` if `GardeningError` prescribed to [`keep_original`] in its [`Doom`]
//! implementation, but let's keep our examples as simple as possible for now).
//!
//! ##### A [`Stack`] stores many [`Entry`]-ies
//!
//! Multiple [`Entry`]-ies can be stacked in a [`Stack`], allowing us to track error
//! propagation. Because every [`Doom`] error can be converted into an [`Entry`],
//! a [`Stack`] can seamlessly archive errors of different types:
//!
//! ```
//! use doomstack::Stack;
//!
//! fn could_go_wrong() -> Result<(), Stack> {
//!     // This could return, e.g., a `Stack` with a `GardeningError` on
//!     // top of a `LandscapingError` on top of a `ShearsError`, each
//!     // archived into an `Entry`.
//!     # Ok(())
//! }
//! ```
//!
//! ##### A [`Top`] is a [`Stack`] with a [`Doom`] on top
//!
//! Between typed [`Doom`]s and non-typed [`Stack`]s, Doomstack offers [`Top`]s.
//! Simply put, a `Top<E>` is a (typed) `E`, sitting on top of a (non-typed)
//! [`Stack`] of [`Entry`]-ies, archiving `E`'s predecessors. `E` is the most
//! recent error, and as such, it is typed, stored on the stack, and ready for
//! exhaustive, programmatic handling. `E`'s predecessors, whose types, variants
//! and fields are less likely to contribute useful error-handling details, are
//! archived in a [`Stack`], tracking the sequence of errors that led to `E`
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
//! fn garden() -> Result<(), Top<GardeningError>> {
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
//! if let Err(top) = garden() {
//!     println!("This time the casualties were {}", top.doom().casualties);
//!     println!("Leading up to the catastrophe:");
//!     for entry in top.stack().entries() {
//!         println!("{:?}", entry);
//!     }
//! }
//! ```
//!
//! Best of both worlds! Our `Top<GardeningError>` stores the (most recent)
//! `GardeningError` (which we can access via the [`Top::doom`] getter), as well as
//! all of `GardeningError`'s predecessors (in their archived form, accessible
//! via the [`Top::stack`] getter). This enables exhaustive error handling, enabled
//! by Rust's powerful type system, for the last, most recent error. For
//! everything that came before that, [`Top`] still archives (at least) every
//! error's [`tag`] and [`description`], as well as (optionally) a [`Location`]
//! indicating where the error was last `spot`ed, which is guaranteed to be
//! meaningful, well-formatted and consistent regardless of compilation profile.
//! You need the original copy of a [`Doom`] deep in a [`Top`]'s predecessors
//! [`Stack`]? No problem: indicate that with [`Doom::keep_original`] and
//! use dynamic dispatch only when you need it!
//!
//! [`tag`]: Doom::tag
//! [`description`]: Doom::description
//! [`keep_original`]: Doom::keep_original
//! [`fot`]: Doom::fot
//! [`archive`]: Entry::archive
//! [`spot`]: ResultExt::spot
//! [`pot`]: ResultExt::pot

mod description;
mod doom;
mod doom_result;
mod entry;
mod here;
mod location;
mod result_ext;
mod stack;
mod top;

use doom_result::DoomResult;

pub use description::Description;
pub use doom::Doom;
pub use entry::Entry;
pub use location::Location;
pub use result_ext::ResultExt;
pub use stack::Stack;
pub use top::Top;

pub use doomstack_derive::Doom;

pub mod prelude {
    pub use crate::{here, Doom, ResultExt, Top};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as doomstack;

    // TODO: Test and fix `#[derive(Doom)]` on empty enums

    #[derive(Debug, PartialEq, Eq, Doom)]
    #[doom(description("Unit `struct` error"))]
    #[doom(wrap(unit_struct_error))]
    struct UnitStructError;

    #[derive(Debug, PartialEq, Eq, Doom)]
    #[doom(description("Empty tuple-like `struct` error"))]
    #[doom(wrap(empty_tuple_struct_error))]
    struct EmptyTupleStructError();

    #[derive(Debug, PartialEq, Eq, Doom)]
    #[doom(description("Single-item tuple-like `struct` error"))]
    #[doom(wrap(single_item_tuple_struct_error))]
    struct SingleItemTupleStructError(u32);

    #[derive(Debug, PartialEq, Eq, Doom)]
    #[doom(description("Multiple-item tuple-like `struct` error"))]
    #[doom(wrap(multiple_item_tuple_struct_error))]
    struct MultipleItemTupleStructError(u32, u64);

    #[derive(Debug, PartialEq, Eq, Doom)]
    #[doom(description("Empty C-style `struct` error"))]
    #[doom(wrap(empty_c_struct_error))]
    struct EmptyCStructError {}

    #[derive(Debug, PartialEq, Eq, Doom)]
    #[doom(description("Single-item C-style `struct` error"))]
    #[doom(wrap(single_item_c_struct_error))]
    struct SingleItemCStructError {
        _x: u32,
    }

    #[derive(Debug, PartialEq, Eq, Doom)]
    #[doom(description("Multiple-item C-style `struct` error"))]
    #[doom(wrap(multiple_item_c_struct_error))]
    struct MultipleItemCStructError {
        _x: u32,
        _y: u64,
    }

    #[test]
    fn wrap_structs() {
        assert_eq!(UnitStructError::unit_struct_error(()), UnitStructError);
        assert_eq!(UnitStructError::unit_struct_error(42u32), UnitStructError);

        assert_eq!(
            UnitStructError::unit_struct_error((1u32, 2u64, "string")),
            UnitStructError
        );

        assert_eq!(
            EmptyTupleStructError::empty_tuple_struct_error(()),
            EmptyTupleStructError()
        );

        assert_eq!(
            SingleItemTupleStructError::single_item_tuple_struct_error(42),
            SingleItemTupleStructError(42)
        );

        assert_eq!(
            MultipleItemTupleStructError::multiple_item_tuple_struct_error((42, 84)),
            MultipleItemTupleStructError(42, 84)
        );

        assert_eq!(
            EmptyCStructError::empty_c_struct_error(()),
            EmptyCStructError {}
        );

        assert_eq!(
            SingleItemCStructError::single_item_c_struct_error(42),
            SingleItemCStructError { _x: 42 }
        );

        assert_eq!(
            MultipleItemCStructError::multiple_item_c_struct_error((42, 84)),
            MultipleItemCStructError { _x: 42, _y: 84 }
        );
    }

    #[derive(Debug, PartialEq, Eq, Doom)]
    enum EnumError {
        #[doom(description("Unit variant"))]
        #[doom(wrap(unit_variant))]
        UnitVariant,
        #[doom(description("Empty tuple-like variant"))]
        #[doom(wrap(empty_tuple_variant))]
        EmptyTupleVariant(),
        #[doom(description("Single-item tuple-like variant"))]
        #[doom(wrap(single_item_tuple_variant))]
        SingleItemTupleVariant(u32),
        #[doom(description("Multiple-item tuple-like variant"))]
        #[doom(wrap(multiple_item_tuple_variant))]
        MultipleItemTupleVariant(u32, u64),
        #[doom(description("Empty C-style variant"))]
        #[doom(wrap(empty_c_variant))]
        EmptyCVariant {},
        #[doom(description("Single-item C-style variant"))]
        #[doom(wrap(single_item_c_variant))]
        SingleItemCVariant { _x: u32 },
        #[doom(description("Multiple-item C-style variant"))]
        #[doom(wrap(multiple_item_c_variant))]
        MultipleItemCVariant { _x: u32, _y: u32 },
    }

    #[test]
    fn wrap_enums() {
        assert_eq!(EnumError::unit_variant(()), EnumError::UnitVariant);
        assert_eq!(EnumError::unit_variant(42u32), EnumError::UnitVariant);

        assert_eq!(
            EnumError::unit_variant((1u32, 2u64, "string")),
            EnumError::UnitVariant
        );

        assert_eq!(
            EnumError::empty_tuple_variant(()),
            EnumError::EmptyTupleVariant()
        );

        assert_eq!(
            EnumError::single_item_tuple_variant(42),
            EnumError::SingleItemTupleVariant(42)
        );

        assert_eq!(
            EnumError::multiple_item_tuple_variant((42, 84)),
            EnumError::MultipleItemTupleVariant(42, 84)
        );

        assert_eq!(EnumError::empty_c_variant(()), EnumError::EmptyCVariant {});

        assert_eq!(
            EnumError::single_item_c_variant(42),
            EnumError::SingleItemCVariant { _x: 42 }
        );

        assert_eq!(
            EnumError::multiple_item_c_variant((42, 84)),
            EnumError::MultipleItemCVariant { _x: 42, _y: 84 }
        );
    }

    #[derive(Doom)]
    #[doom(description("Default `keep_original` struct error"))]
    struct DefaultKeepOriginalStructError;

    #[derive(Doom)]
    #[doom(description("Tagged `keep_original` struct error"))]
    #[doom(keep_original)]
    struct TaggedKeepOriginalStructError;

    #[derive(Doom)]
    #[doom(description("Conditional `keep_original` struct error"))]
    #[doom(keep_original(*_severity > 42))]
    struct ConditionalKeepOriginalStructError {
        _severity: u32,
    }

    #[test]
    fn keep_original_structs() {
        let default_entry = Entry::archive(DefaultKeepOriginalStructError);
        assert!(default_entry.original().is_none());

        let tagged_entry = Entry::archive(TaggedKeepOriginalStructError);
        assert!(tagged_entry.original().is_some());

        let unsatisfied_conditional_entry =
            Entry::archive(ConditionalKeepOriginalStructError { _severity: 12 });

        assert!(unsatisfied_conditional_entry.original().is_none());

        let satisfied_conditional_entry =
            Entry::archive(ConditionalKeepOriginalStructError { _severity: 120 });

        assert!(satisfied_conditional_entry.original().is_some());
    }

    #[derive(Doom)]
    enum KeepOriginalEnumError {
        #[doom(description("Default `keep_original` variant"))]
        DefaultVariant,
        #[doom(description("Tagged `keep_original` variant"))]
        #[doom(keep_original)]
        TaggedVariant,
        #[doom(description("Conditional `keep_original` variant"))]
        #[doom(keep_original(*_severity > 42))]
        ConditionalVariant { _severity: u32 },
    }

    #[test]
    fn keep_original_enums() {
        let default_entry = Entry::archive(KeepOriginalEnumError::DefaultVariant);
        assert!(default_entry.original().is_none());

        let tagged_entry = Entry::archive(KeepOriginalEnumError::TaggedVariant);
        assert!(tagged_entry.original().is_some());

        let unsatisfied_conditional_entry =
            Entry::archive(KeepOriginalEnumError::ConditionalVariant { _severity: 12 });

        assert!(unsatisfied_conditional_entry.original().is_none());

        let satisfied_conditional_entry =
            Entry::archive(KeepOriginalEnumError::ConditionalVariant { _severity: 120 });

        assert!(satisfied_conditional_entry.original().is_some());
    }
}
