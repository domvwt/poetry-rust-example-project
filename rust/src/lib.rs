use pyo3::prelude::*;

#[pymodule]
fn addersrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(sum_list, m)?)?;
    Ok(())
}

#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pyfunction]
fn sum_list(input_list: Vec<i32>) -> i32 {
    input_list.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Unit tests for add
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
    }

    // Unit tests for sum_list
    #[test]
    fn test_sum_list() {
        assert_eq!(sum_list(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_list(vec![-1, -2, -3, -4, -5]), -15);
        assert_eq!(sum_list(vec![1, -1, 2, -2, 3, -3]), 0);
    }
}
