#![allow(dead_code)]
use crate::{cell::CellType, field::Field};

pub struct TerminalViewer;

impl TerminalViewer {
    pub(crate) fn view(field: &Field) -> String {
        let width = field.cells.len() / field.height as usize;
        field
            .cells
            .chunks(width)
            .map(|row| {
                row.iter()
                    .map(|c| match c._type {
                        CellType::Bomb => "*".to_string(),
                        CellType::Empty(b) => b.to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub(crate) fn view_only_opened(field: &Field) -> String {
        let width = field.cells.len() / field.height as usize;
        field
            .cells
            .chunks(width)
            .map(|row| {
                row.iter()
                    .map(|c| {
                        if c.is_opened() {
                            match c._type {
                                CellType::Bomb => "*".to_string(),
                                CellType::Empty(b) => b.to_string(),
                            }
                        } else {
                            "█".to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
