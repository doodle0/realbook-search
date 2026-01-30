/// Navigation utilities for keyboard result navigation

/// Calculate the next index when navigating down through results
/// Wraps around to 0 if at the end
pub fn next_result_index(current: Option<usize>, total_results: usize) -> usize {
    if total_results == 0 {
        return 0;
    }

    match current {
        None => 0,
        Some(idx) => {
            if idx + 1 >= total_results {
                0
            } else {
                idx + 1
            }
        }
    }
}

/// Calculate the previous index when navigating up through results
/// Wraps around to last item if at the beginning
pub fn prev_result_index(current: Option<usize>, total_results: usize) -> usize {
    if total_results == 0 {
        return 0;
    }

    match current {
        None => total_results - 1,
        Some(idx) => {
            if idx == 0 {
                total_results - 1
            } else {
                idx - 1
            }
        }
    }
}
