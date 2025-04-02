pub mod algorithms;
pub mod movements;
pub mod pieces;

use movements::{CenterMove, CubeMove, LateralMove, RotationalMove};
use pieces::{Center, Color, Corner, Edge, Face, StickerRef};

pub struct Cube {
    /// Size of the cube (e.g., 3 for 3x3x3)
    size: usize,
    /// Corner pieces (always 8 for any size)
    corners: [Corner; 8],
    /// Edge pieces (12*(n-1) total for an nxnxn cube)
    edges: Vec<Edge>,
    /// Center pieces (6*(n-2)² total for an nxnxn cube where n>2)
    centers: Vec<Center>,
    /// Faces (visual representation - always 6 for any size)
    faces: [Face; 6],
}

impl Cube {
    /// Creates a new solved cube of the specified size
    pub fn new(size: usize) -> Self {
        assert!(size >= 2, "Cube size must be at least 2");

        // Create corners (same for all cube sizes)
        let corners = Self::init_corners();

        // Create edges (depends on cube size)
        let edges = Self::init_edges(size);

        // Create centers (depends on cube size)
        let centers = Self::init_centers(size);

        // Initialize the cube without faces first
        let mut cube = Self {
            size,
            corners,
            edges,
            centers,
            faces: Self::init_empty_faces(size),
        };

        // Now set up the faces with proper sticker references
        cube.init_face_stickers();

        cube
    }

    /// Initialize empty faces
    fn init_empty_faces(size: usize) -> [Face; 6] {
        [
            Face::new(0, size), // Up
            Face::new(1, size), // Down
            Face::new(2, size), // Front
            Face::new(3, size), // Back
            Face::new(4, size), // Left
            Face::new(5, size), // Right
        ]
    }

    /// Initialize the sticker references for all faces
    fn init_face_stickers(&mut self) {
        // This is a complex function that sets up which pieces are visible
        // from which faces and in what orientation

        // For each face:
        // 1. Determine which corners are visible (and which color)
        // 2. Determine which edges are visible (and which color)
        // 3. Determine which centers are visible

        // As a simplified example for a 3x3x3:
        // Up face (0):
        //   Corners: 0,1,2,3 (with specific color indices)
        //   Edges: 0,1,2,3 (with specific color indices)
        //   Center: 0

        // This will be a large function with a lot of mapping logic
        unimplemented!()
    }

    /// Initialize the 8 corner pieces in their solved state
    fn init_corners() -> [Corner; 8] {
        // Define the colors for each corner in the solved state
        // The order of colors is important and should be consistent
        // For example: [U/D, F/B, L/R] where each is the face color

        [
            // URF - 0
            Corner {
                colors: [Color::White, Color::Red, Color::Green],
                orientation: 0,
            },
            // UFL - 1
            Corner {
                colors: [Color::White, Color::Red, Color::Blue],
                orientation: 0,
            },
            // ULB - 2
            Corner {
                colors: [Color::White, Color::Blue, Color::Orange],
                orientation: 0,
            },
            // UBR - 3
            Corner {
                colors: [Color::White, Color::Orange, Color::Green],
                orientation: 0,
            },
            // DFR - 4
            Corner {
                colors: [Color::Yellow, Color::Red, Color::Green],
                orientation: 0,
            },
            // DLF - 5
            Corner {
                colors: [Color::Yellow, Color::Blue, Color::Red],
                orientation: 0,
            },
            // DBL - 6
            Corner {
                colors: [Color::Yellow, Color::Orange, Color::Blue],
                orientation: 0,
            },
            // DRB - 7
            Corner {
                colors: [Color::Yellow, Color::Green, Color::Orange],
                orientation: 0,
            },
        ]
    }

    /// Initialize the edge pieces for a cube of given size
    fn init_edges(size: usize) -> Vec<Edge> {
        // For a cube of size n, there are 12*(n-1) edges
        let num_edges = 12 * (size - 1);
        let mut edges = Vec::with_capacity(num_edges);

        // Define the colors for each edge in the solved state
        // For a 3x3 cube, we'd have 12 edges

        // For an NxN cube where N>3, we'd have multiple edges along each edge line
        // For example:
        // - UR edge would become "UR1, UR2, ..., UR(N-1)" from left to right
        // - Front-Right edge would become "FR1, FR2, ..., FR(N-1)" from top to bottom

        // This approach gets complex for larger cubes, especially for
        // tracking multiple edges of the same type

        // As a simplified implementation for now:
        if size == 3 {
            // Standard 3x3 edges
            edges.push(Edge {
                colors: [Color::White, Color::Red],
                orientation: 0,
            }); // UF
            edges.push(Edge {
                colors: [Color::White, Color::Green],
                orientation: 0,
            }); // UR
            edges.push(Edge {
                colors: [Color::White, Color::Blue],
                orientation: 0,
            }); // UL
            edges.push(Edge {
                colors: [Color::White, Color::Orange],
                orientation: 0,
            }); // UB

            edges.push(Edge {
                colors: [Color::Red, Color::Green],
                orientation: 0,
            }); // FR
            edges.push(Edge {
                colors: [Color::Red, Color::Blue],
                orientation: 0,
            }); // FL
            edges.push(Edge {
                colors: [Color::Orange, Color::Blue],
                orientation: 0,
            }); // BL
            edges.push(Edge {
                colors: [Color::Orange, Color::Green],
                orientation: 0,
            }); // BR

            edges.push(Edge {
                colors: [Color::Yellow, Color::Red],
                orientation: 0,
            }); // DF
            edges.push(Edge {
                colors: [Color::Yellow, Color::Green],
                orientation: 0,
            }); // DR
            edges.push(Edge {
                colors: [Color::Yellow, Color::Blue],
                orientation: 0,
            }); // DL
            edges.push(Edge {
                colors: [Color::Yellow, Color::Orange],
                orientation: 0,
            }); // DB
        } else {
            // For other cube sizes, we'd need a more complex edge initialization
            unimplemented!()
        }

        edges
    }

    /// Initialize the center pieces for a cube of given size
    fn init_centers(size: usize) -> Vec<Center> {
        if size == 2 {
            // 2x2x2 has no centers
            return Vec::new();
        }

        // For a cube of size n, there are 6*(n-2)² centers
        let centers_per_face = (size - 2) * (size - 2);
        let total_centers = 6 * centers_per_face;
        let mut centers = Vec::with_capacity(total_centers);

        // Define the 6 face colors
        let face_colors = [
            Color::White,  // Up
            Color::Yellow, // Down
            Color::Red,    // Front
            Color::Orange, // Back
            Color::Blue,   // Left
            Color::Green,  // Right
        ];

        // Create centers for each face
        for face in 0..6 {
            for _ in 0..(centers_per_face) {
                centers.push(Center {
                    color: face_colors[face],
                });
            }
        }

        centers
    }

    /// Checks if the cube is solved
    pub fn is_solved(&self) -> bool {
        // A cube is solved when all pieces are in their correct positions
        // and have the correct orientations

        // Check corners
        if !self.are_corners_solved() {
            return false;
        }

        // Check edges
        if !self.are_edges_solved() {
            return false;
        }

        // Check centers
        if !self.are_centers_solved() {
            return false;
        }

        true
    }

    /// Helper method to check if corners are solved
    fn are_corners_solved(&self) -> bool {
        // Implementation to check if all corners are in correct positions
        // and orientations
        unimplemented!()
    }

    /// Helper method to check if edges are solved
    fn are_edges_solved(&self) -> bool {
        // Implementation to check if all edges are in correct positions
        // and orientations
        unimplemented!()
    }

    /// Helper method to check if centers are solved
    fn are_centers_solved(&self) -> bool {
        // For 2x2x2 cubes (no centers)
        if self.size == 2 {
            return true;
        }

        // Implementation to check if all centers are in correct positions
        unimplemented!()
    }

    /// Get a reference to a specific face
    pub fn get_face(&self, face_id: usize) -> &Face {
        &self.faces[face_id]
    }

    /// After any move, update all faces to reflect the new state
    fn update_faces(&mut self) {
        // Instead of trying to update each face with a reference to self,
        // we'll gather all the information needed first, then update the faces

        // Create a mapping of all piece data that the faces need
        let corner_data: Vec<_> = self
            .corners
            .iter()
            .map(|corner| (corner.colors, corner.orientation))
            .collect();

        let edge_data: Vec<_> = self
            .edges
            .iter()
            .map(|edge| (edge.colors, edge.orientation))
            .collect();

        let center_data: Vec<_> = self.centers.iter().map(|center| center.color).collect();

        // Now update each face with the collected data
        for face in &mut self.faces {
            face.update_with_piece_data(&corner_data, &edge_data, &center_data);
        }
    }

    /// Render a face as a 2D grid of colors (useful for display)
    pub fn render_face(&self, face_id: usize) -> Vec<Vec<Color>> {
        let face = &self.faces[face_id];
        let mut colors = Vec::with_capacity(face.size);

        // Prepare the piece data first
        let corner_data: Vec<_> = self
            .corners
            .iter()
            .map(|corner| (corner.colors, corner.orientation))
            .collect();

        let edge_data: Vec<_> = self
            .edges
            .iter()
            .map(|edge| (edge.colors, edge.orientation))
            .collect();

        let center_data: Vec<_> = self.centers.iter().map(|center| center.color).collect();

        // Now render the face
        for row in 0..face.size {
            let mut row_colors = Vec::with_capacity(face.size);
            for col in 0..face.size {
                let color = face.get_color(row, col, &corner_data, &edge_data, &center_data);
                row_colors.push(color);
            }
            colors.push(row_colors);
        }

        colors
    }
}
