use crate::prelude::*;
use petgraph::graphmap::GraphMap;
use petgraph::Undirected;

type Grid = GraphMap<Coordinates, (), Undirected>;

/// A collection of [`Field`]s with passages between them.
///
/// Use one of the provided [`Generator`]s to create an instance of this type.
pub struct Maze {
    pub(crate) grid: Grid,
    /// At which coordinates the start field lies
    pub start: Coordinates,
    /// At which coordinates the goal field lies
    pub goal: Coordinates,
    /// How large the maze is in (width, height) format
    pub size: (i32, i32),
}

impl Maze {
    pub(crate) fn new(width: i32, height: i32, start: Coordinates, goal: Coordinates) -> Self {
        Maze {
            grid: GraphMap::with_capacity((width * height) as usize, 0),
            size: (width, height),
            start,
            goal,
        }
    }

    /// Retrieve the [`Field`] which is located at `coordinates`
    pub fn get_field(&self, coordinates: &Coordinates) -> Option<Field> {
        if self.are_coordinates_inside(coordinates) {
            // figure out in which directions passages exist
            let passages: Vec<_> = Direction::all()
                .iter()
                .filter(|dir| {
                    self.grid
                        .contains_edge(coordinates.clone(), coordinates.next(dir))
                })
                .map(|dir| dir.clone())
                .collect();

            // TODO set field type correctly
            Some(Field::new(FieldType::Normal, coordinates.clone(), passages))
        } else {
            None
        }
    }

    pub(crate) fn are_coordinates_inside(&self, coordinates: &Coordinates) -> bool {
        coordinates.x >= 0
            && coordinates.x < self.size.0
            && coordinates.y >= 0
            && coordinates.y < self.size.1
    }
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for iy in 0..self.size.1 {
            // print top passage
            for ix in 0..self.size.0 {
                f.write_str("·")?;
                if self
                    .get_field(&(ix, iy).into())
                    .unwrap()
                    .has_passage(&Direction::North)
                {
                    f.write_str(" ")?;
                } else {
                    f.write_str("-")?;
                }
            }
            f.write_str("·\n")?;

            // print left passage and room icon
            for ix in 0..self.size.0 {
                let field = self.get_field(&(ix, iy).into()).unwrap();
                if field.has_passage(&Direction::West) {
                    f.write_str(" ")?;
                } else {
                    f.write_str("|")?;
                }

                if self.start == (ix, iy).into() {
                    f.write_str("S")?;
                } else if self.goal == (ix, iy).into() {
                    f.write_str("G")?;
                } else {
                    f.write_str(" ")?;
                }
            }
            f.write_str("|\n")?;

            // print bottom line
            if iy == self.size.1 - 1 {
                for _ix in 0..self.size.0 {
                    f.write_str("·-")?;
                }
                f.write_str("·\n")?;
            }
        }

        Ok(())
    }
}
