/// Represents a color on the Rubik's Cube
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

/// Represents a corner piece of the Rubik's Cube
///
/// A corner piece has 3 stickers (one from each of three adjacent faces)
/// and can be in 8 possible positions with 3 possible orientations each
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Corner {
    /// The colors of the corner piece (3 colors)
    pub colors: [Color; 3],
    /// The orientation of the corner (0-2)
    pub orientation: usize,
}

/// Represents an edge piece of the Rubik's Cube
///
/// An edge piece has 2 stickers (one from each of two adjacent faces)
/// and can be in 12 possible positions with 2 possible orientations each
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    /// The colors of the edge piece (2 colors)
    pub colors: [Color; 2],
    /// The orientation of the edge (0-1)
    pub orientation: usize,
}

/// Represents a center piece of the Rubik's Cube
///
/// A center piece has 1 sticker (from a single face)
/// For a 3x3x3 cube, there are 6 centers (one per face)
/// For a NxNxN cube (where N>3), there are 6(N-2)² centers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Center {
    /// The color of the center piece
    pub color: Color,
}

/// A reference to a specific sticker/color on the cube
/// This is used by the Face struct to track which stickers are visible
#[derive(Debug, Clone, Copy)]
pub enum StickerRef {
    /// Reference to a corner piece sticker (corner_index, color_index)
    Corner(usize, usize),
    /// Reference to an edge piece sticker (edge_index, color_index)
    Edge(usize, usize),
    /// Reference to a center piece (center_index)
    Center(usize),
}

/// Represents a face of the Rubik's Cube (what is visually seen from one side)
///
/// This is a "view" into the current state of the cube, not a physical part
#[derive(Debug, Clone)]
pub struct Face {
    /// Which face this is (0-5 corresponding to Up, Down, Front, Back, Left, Right)
    pub face_id: usize,

    /// Size of the face (N for an NxN cube)
    pub size: usize,

    /// References to all stickers visible on this face
    /// Arranged in a 2D grid: stickers[row][col]
    /// Each sticker is a reference to a piece and which color to show
    pub stickers: Vec<Vec<StickerRef>>,
}

impl Face {
    /// Create a new face for a cube of the given size
    pub fn new(face_id: usize, size: usize) -> Self {
        let mut stickers = Vec::with_capacity(size);

        // We'll initialize with empty vectors - the actual sticker
        // references will be set up when the cube is created
        for _ in 0..size {
            let row = Vec::with_capacity(size);
            stickers.push(row);
        }

        Self {
            face_id,
            size,
            stickers,
        }
    }

    /// Update this face using the provided piece data
    pub fn update_with_piece_data(
        &mut self,
        corner_data: &[(([Color; 3], usize))],
        edge_data: &[(([Color; 2], usize))],
        center_data: &[Color],
    ) {
        // For each sticker reference in this face, update it based on the current piece data
        for row in 0..self.size {
            for col in 0..self.size {
                if row >= self.stickers.len() || col >= self.stickers[row].len() {
                    continue; // Skip if we don't have this position initialized
                }

                match self.stickers[row][col] {
                    StickerRef::Corner(corner_idx, color_idx) => {
                        // The sticker reference itself stays the same - it still points
                        // to the same corner and color index. The actual color will be different
                        // when rendered because the corner's orientation might have changed.
                    }
                    StickerRef::Edge(edge_idx, color_idx) => {
                        // Same as with corners - the reference doesn't change,
                        // but the rendered color will, due to orientation changes
                    }
                    StickerRef::Center(center_idx) => {
                        // Centers don't change orientation, just position
                        // For fixed centers (like in a 3x3x3), this doesn't matter
                        // For larger cubes, the center_idx reference may need to be updated
                    }
                }
            }
        }
    }

    /// Get the color currently showing at a specific position on the face
    pub fn get_color(
        &self,
        row: usize,
        col: usize,
        corner_data: &[(([Color; 3], usize))],
        edge_data: &[(([Color; 2], usize))],
        center_data: &[Color],
    ) -> Color {
        assert!(row < self.size && col < self.size, "Invalid position");

        if row >= self.stickers.len() || col >= self.stickers[row].len() {
            // Default to white if position isn't initialized
            return Color::White;
        }

        match self.stickers[row][col] {
            StickerRef::Corner(corner_idx, color_idx) => {
                // Apply orientation to get the correct color
                let (colors, orientation) = &corner_data[corner_idx];
                let adjusted_idx = (color_idx + orientation) % 3;
                colors[adjusted_idx]
            }
            StickerRef::Edge(edge_idx, color_idx) => {
                // Apply orientation to get the correct color
                let (colors, orientation) = &edge_data[edge_idx];
                let adjusted_idx = (color_idx + orientation) % 2;
                colors[adjusted_idx]
            }
            StickerRef::Center(center_idx) => center_data[center_idx],
        }
    }
}
