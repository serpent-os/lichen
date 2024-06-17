// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use ratatui::{
    layout::{Constraint, Rect},
    widgets::Padding,
};

pub struct Layout {
    pub area: Rect,
    pub children: Vec<Layout>,
}

pub fn pad_constraint(constraint: Constraint, padding: u16) -> Constraint {
    match constraint {
        Constraint::Min(x) => Constraint::Min(x + padding),
        Constraint::Max(x) => Constraint::Max(x + padding),
        Constraint::Length(x) => Constraint::Length(x + padding),
        c => c,
    }
}

pub fn pad_rect(rect: Rect, padding: Padding) -> Rect {
    let horizontal = padding.left + padding.right;
    let vertical = padding.top + padding.bottom;

    if rect.width < horizontal || rect.height < vertical {
        Rect::ZERO
    } else {
        Rect {
            x: rect.x.saturating_add(padding.left),
            y: rect.y.saturating_add(padding.top),
            width: rect.width.saturating_sub(horizontal),
            height: rect.height.saturating_sub(vertical),
        }
    }
}
