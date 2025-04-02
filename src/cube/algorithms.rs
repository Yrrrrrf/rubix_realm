use crate::cube::movements::CubeMove;

/// Execute a sexy move (R U R' U')
pub fn sexy_move<T: CubeMove>(cube: &mut T) -> &mut T {
    cube.move_cube("R U R' U'")
}

/// Execute a sledgehammer (R' F R F')
pub fn sledgehammer<T: CubeMove>(cube: &mut T) -> &mut T {
    cube.move_cube("R' F R F'")
}

/// Execute a sune algorithm (R U R' U R U2 R')
pub fn sune<T: CubeMove>(cube: &mut T) -> &mut T {
    cube.move_cube("R U R' U R U2 R'")
}
