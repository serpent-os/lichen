// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Lichen cleanup steps
//! Despite the fact we could trivially implement as a `Drop` or some
//! other Rust idiom, we still need to provide active feedback to the user
//! for whatever step is currently running.
//!
//! To that effect we provide a mirror of [`Step`] by way of a Cleanup.

use super::{partitions, Context};

/// Encapsulate the cleanup stages
pub enum Cleanup {
    /// Unmount a mountpoint
    Unmount(Box<partitions::Unmount>),
}

impl<'a> Cleanup {
    /// Create new unmount cleanup stage
    pub fn unmount(unmount: partitions::Unmount) -> Self {
        Self::Unmount(Box::new(unmount))
    }

    /// Return cleanup step title
    pub fn title(&self) -> String {
        match &self {
            Self::Unmount(s) => s.title(),
        }
    }

    /// Fully describe cleanup step
    pub fn describe(&self) -> String {
        match &self {
            Cleanup::Unmount(s) => s.describe(),
        }
    }

    /// Execute the cleanup step
    pub async fn execute(&self, context: &impl Context<'a>) -> Result<(), super::Error> {
        match &self {
            Cleanup::Unmount(s) => Ok(s.execute(context).await?),
        }
    }
}
