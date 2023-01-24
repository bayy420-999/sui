// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

pub use sui_proc_macros::*;

/// Evaluates an expression in a new thread which will not be subject to interception of
/// getrandom(), clock_gettime(), etc.
#[cfg(msim)]
#[macro_export]
macro_rules! nondeterministic {
    ($expr: expr) => {
        std::thread::scope(move |s| s.spawn(move || $expr).join().unwrap())
    };
}

/// Simply evaluates expr.
#[cfg(not(msim))]
#[macro_export]
macro_rules! nondeterministic {
    ($expr: expr) => {
        $expr
    };
}

#[macro_export]
macro_rules! maybe_kill_node {
    ($probability: expr) => {
        #[cfg(msim)]
        {
            let probability: f64 = $probability;

            use rand::Rng;
            if msim::rand::thread_rng().gen_range(0.0..1.0) < probability {
                msim::task::kill_current_node(None);
            }
        }
    };
}
