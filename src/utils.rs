///
/// Whether the coordinate is out of bound or not
/// 
/// # Parameters
/// 
/// * x: x coordinate
/// * y: y coordinate
/// * field_size: x,y max size (square)
/// 
/// # Returns
/// 
/// * bool: Return if x,y are not out of bound
pub fn is_not_out_of_bound(x: i8, y: i8, field_size: i8) -> bool {
    x >= 0 && x < field_size && y >= 0 && y < field_size
}
