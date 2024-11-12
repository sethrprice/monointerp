use core::fmt::Display;

use num_traits::Num;

pub fn monointerp<T>(query: &[T], x: &[T], y: &[T]) -> Result<Vec<T>, String>
where
    T: PartialOrd + Copy + Num + Display,
{
    let query_length = query.len();
    let axis_length = x.len();

    // assert that x and y are of same length
    if axis_length != y.len() {
        let error_message = format!(
            "length of x ({}) must equal length of y ({})",
            axis_length,
            y.len()
        );
        return Err(error_message);
    }

    // Find start

    // if left starts before x, error
    if query[0] < x[0] {
        let error_string = format!("query ({}) is below x range ({})", query[0], x[0]);
        return Err(error_string);
    }

    // do binary search to find the start index
    let mut j = binary_search(query[0], x);

    // if our start index is bigger than the length, error
    if j + 1 >= axis_length {
        let error_message = format!(
            "query ({}) is above x range ({})",
            query[0],
            x[axis_length - 1]
        );
        return Err(error_message);
    }

    // assumptions:
    //  left > right
    //  x, left, and right are all monotonically increasing

    // do the interpolation
    let mut interpolated_vec: Vec<T> = Vec::with_capacity(query_length);
    for i in 0..query.len() {
        // advance indices
        while query[i] > x[j + 1] && j < axis_length {
            j += 1;
        }

        // bounds check
        if j == axis_length {
            let error_message = format!("query ({}) is outside range (...{})", query[i], x[j]);
            return Err(error_message);
        }

        // get weighting
        let weight = (query[i] - x[j]) / (x[j + 1] - x[j]);

        // do interpolation
        let y_new = (y[j + 1] - y[j]) * weight + y[j];

        interpolated_vec.push(y_new);
    }

    Ok(interpolated_vec)
}

fn binary_search<T>(key: T, arr: &[T]) -> usize
where
    T: PartialOrd + Copy,
{
    let mut imin: usize = 0;
    let mut imax: usize = arr.len();

    if key > arr[imax - 1] {
        return imax;
    }

    while imin < imax {
        let imid = imin + ((imax - imin) >> 1);
        if key >= arr[imid] {
            imin = imid + 1;
        } else {
            imax = imid;
        }
    }
    return imin.saturating_sub(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let v = vec![0.0, 1.0, 2.0];
        let s = binary_search(1.0, &v);
        assert_eq!(s, 1);
    }

    #[test]
    fn test_binary_search_absent() {
        let v = vec![0, 1, 3, 4];
        let s = binary_search(2, &v);
        assert_eq!(s, 1);
    }

    #[test]
    fn test_interp_simple() {
        let xp = vec![0., 1., 2., 3.];
        let fp = vec![4., 3., 2., 1.];
        let query = vec![1., 2.];
        let expected_output = vec![3., 2.];
        let output = monointerp(&query, &xp, &fp).unwrap();
        assert_eq!(expected_output, output);
    }

    #[test]
    fn test_interp_harder() {
        let xp = vec![1., 2., 3.];
        let fp = vec![3., 2., 0.];
        let query = vec![1., 1.5, 2.72, 3.];
        let expected_output = vec![3., 2.5, 0.56, 0.];
        let output = monointerp(&query, &xp, &fp).unwrap();
        for (o, e) in output.iter().zip(expected_output.iter()) {
            let diff: f64 = o - e;
            assert!(diff.abs() < 0.00001);
        }
    }
}
