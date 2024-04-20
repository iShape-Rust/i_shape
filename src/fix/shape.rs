use serde::{Serialize, Deserialize};
use crate::fix::path::{FixPath, FixPathExtension};
use crate::fix::paths::FixPathsExtension;

/// Represents a fixed geometric shape with contour and holes.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FixShape {
    pub paths: Vec<FixPath>,
}

impl FixShape {

    pub fn points_count(&self) -> usize {
        self.paths.points_count()
    }
    /// Determines if the shape represents a convex polygon.
    ///
    /// # Returns
    /// `true` if the shape is a convex polygon, `false` otherwise.
    pub fn is_convex_polygon(&self) -> bool {
        self.paths.len() == 1 && self.contour().is_convex()
    }

    /// Returns the contour defining the outer boundary of the shape.
    /// Assumes the first path in `paths` is the contour.
    ///
    /// # Returns
    /// A reference to the `FixPath` representing the contour.
    pub fn contour(&self) -> &FixPath {
        &self.paths[0]
    }

    /// Returns the array of holes defining the inner boundaries of the shape.
    ///
    /// # Returns
    /// A slice of `FixPath` representing the holes.
    pub fn holes(&self) -> &[FixPath] {
        &self.paths[1..]
    }

    /// Initializes a new shape with the specified paths.
    /// The first path is used as the contour, and remaining paths as holes.
    /// Assumes that paths are properly ordered beforehand.
    ///
    /// # Parameters
    /// * `paths`: A `Vec<FixPath>` defining the contour and holes.
    ///
    /// # Returns
    /// A new instance of `FixShape`.
    pub fn new(paths: Vec<FixPath>) -> Self {
        Self { paths }
    }
}