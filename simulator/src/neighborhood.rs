
/// Returns neighbors of the top left corner cell
///
/// ## 4 x 4 Grid Example:
/// 
/// #### (i, j) = (0, 0)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | ce | n4 | .. | n3 |
/// | n6 | n7 | .. | n5 |
/// | .. | .. | .. | .. |
/// | n1 | n2 | .. | n0 |
/// 
pub(super) fn top_left(i_max: usize, j_max: usize) -> [(usize, usize); 8] {
  [
    (i_max, j_max),
    (i_max, 0),
    (i_max, 1),
    (0, j_max),
    (0, 1),
    (1, j_max),
    (1, 0),
    (1, 1),
  ]
}

/// Returns neighbors of the top edge cells
///
/// ## 4 x 4 Grid Examples:
/// 
/// #### (i, j) = (0, 1)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n3 | ce | n4 | .. |
/// | n5 | n6 | n7 | .. |
/// | .. | .. | .. | .. |
/// | n0 | n1 | n2 | .. |
/// 
/// #### (i, j) = (0, 2)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | .. | n3 | ce | n4 |
/// | .. | n5 | n6 | n7 |
/// | .. | .. | .. | .. |
/// | .. | n0 | n1 | n2 |
/// 
pub(super) fn top_edge(i_max: usize, j: usize) -> [(usize, usize); 8] {
  [
    (i_max, j - 1),
    (i_max, j),
    (i_max, j + 1),
    (0, j - 1),
    (0, j + 1),
    (1, j - 1),
    (1, j),
    (1, j + 1),
  ]
}

/// Returns neighbors of the top right corner cell
///
/// ## 4 x 4 Grid Example:
/// 
/// #### (i, j) = (0, 3)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n4 | .. | n3 | ce |
/// | n7 | .. | n5 | n6 |
/// | .. | .. | .. | .. |
/// | n2 | .. | n0 | n1 |
/// 
pub(super) fn top_right(i_max: usize, j_max: usize) -> [(usize, usize); 8] {
  [
    (i_max, j_max - 1),
    (i_max, j_max),
    (i_max, 0),
    (0, j_max - 1),
    (0, 0),
    (1, j_max - 1),
    (1, j_max),
    (1, 0),
  ]
}

/// Returns neighbors of the left edge cells
///
/// ## 4 x 4 Grid Examples:
/// 
/// #### (i, j) = (1, 0)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n1 | n2 | .. | n0 |
/// | ce | n4 | .. | n3 |
/// | n6 | n7 | .. | n5 |
/// | .. | .. | .. | .. |
/// 
/// #### (i, j) = (2, 0)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | .. | .. | .. | .. |
/// | n1 | n2 | .. | n0 |
/// | ce | n4 | .. | n3 |
/// | n6 | n7 | .. | n5 |
/// 
pub(super) fn left_edge(i: usize, j_max: usize) -> [(usize, usize); 8] {
  [
    (i - 1, j_max),
    (i - 1, 0),
    (i - 1, 1),
    (i, j_max),
    (i, 1),
    (i + 1, j_max),
    (i + 1, 0),
    (i + 1, 1),
  ]
}

/// Returns neighbors of the middle cells
///
/// ## 4 x 4 Grid Examples:
/// 
/// #### (i, j) = (1, 1)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n0 | n1 | n2 | .. |
/// | n3 | ce | n4 | .. |
/// | n5 | n6 | n7 | .. |
/// | .. | .. | .. | .. |
/// 
/// #### (i, j) = (1, 2)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | .. | n0 | n1 | n2 |
/// | .. | n3 | ce | n4 |
/// | .. | n5 | n6 | n7 |
/// | .. | .. | .. | .. |
/// 
/// #### (i, j) = (2, 1)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | .. | .. | .. | .. |
/// | n0 | n1 | n2 | .. |
/// | n3 | ce | n4 | .. |
/// | n5 | n6 | n7 | .. |
/// 
/// #### (i, j) = (2, 2)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | .. | .. | .. | .. |
/// | .. | n0 | n1 | n2 |
/// | .. | n3 | ce | n4 |
/// | .. | n5 | n6 | n7 |
/// 
pub(super) fn mid(i: usize, j: usize) -> [(usize, usize); 8] {
  [
    (i - 1, j - 1),
    (i - 1, j),
    (i - 1, j + 1),
    (i, j - 1),
    (i, j + 1),
    (i + 1, j - 1),
    (i + 1, j),
    (i + 1, j + 1),
  ]
}

/// Returns neighbors of the right edge cells
///
/// ## 4 x 4 Grid Examples:
/// 
/// #### (i, j) = (1, 3)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n2 | .. | n0 | n1 |
/// | n4 | .. | n3 | ce |
/// | n7 | .. | n5 | n6 |
/// | .. | .. | .. | .. |
/// 
/// #### (i, j) = (2, 3)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | .. | .. | .. | .. |
/// | n2 | .. | n0 | n1 |
/// | n4 | .. | n3 | ce |
/// | n7 | .. | n5 | n6 |
/// 
pub(super) fn right_edge(i: usize, j_max: usize) -> [(usize, usize); 8] {
  [
    (i - 1, j_max - 1),
    (i - 1, j_max),
    (i - 1, 0),
    (i, j_max - 1),
    (i, 0),
    (i + 1, j_max - 1),
    (i + 1, j_max),
    (i + 1, 0),
  ]
}

/// Returns neighbors of the bot left corner cell
///
/// ## 4 x 4 Grid Example:
/// 
/// #### (i, j) = (3, 0)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n6 | n7 | .. | n5 |
/// | .. | .. | .. | .. |
/// | n1 | n2 | .. | n0 |
/// | ce | n4 | .. | n3 |
/// 
pub(super) fn bot_left(i_max: usize, j_max: usize) -> [(usize, usize); 8] {
  [
    (i_max - 1, j_max),
    (i_max - 1, 0),
    (i_max - 1, 1),
    (i_max, j_max),
    (i_max, 1),
    (0, j_max),
    (0, 0),
    (0, 1),
  ]
}

/// Returns neighbors of the top edge cells
/// 
/// ## 4 x 4 Grid Examples:
/// 
/// #### (i, j) = (3, 1)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n5 | n6 | n7 | .. |
/// | .. | .. | .. | .. |
/// | n0 | n1 | n2 | .. |
/// | n3 | ce | n4 | .. |
/// 
/// #### (i, j) = (3, 2)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | .. | n5 | n6 | n7 |
/// | .. | .. | .. | .. |
/// | .. | n0 | n1 | n2 |
/// | .. | n3 | ce | n4 |
/// 
pub(super) fn bot_edge(i_max: usize, j: usize) -> [(usize, usize); 8] {
  [
    (i_max - 1, j - 1),
    (i_max - 1, j),
    (i_max - 1, j + 1),
    (i_max, j - 1),
    (i_max, j + 1),
    (0, j - 1),
    (0, j),
    (0, j + 1),
  ]
}

/// Returns neighbors of the bot right corner cell
///
/// ## 4 x 4 Grid Example:
/// 
/// #### (i, j) = (3, 3)
/// 
/// |    |    |    |    |
/// |----|----|----|----|
/// | n7 | .. | n5 | n6 |
/// | .. | .. | .. | .. |
/// | n2 | .. | n0 | n1 |
/// | n4 | .. | n3 | ce |
/// 
pub(super) fn bot_right(i_max: usize, j_max: usize) -> [(usize, usize); 8] {
  [
    (i_max - 1, j_max - 1),
    (i_max - 1, j_max),
    (i_max - 1, 0),
    (i_max, j_max - 1),
    (i_max, 0),
    (0, j_max - 1),
    (0, j_max),
    (0, 0),
  ]
}
