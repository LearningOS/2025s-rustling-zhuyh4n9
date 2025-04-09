/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
    where T: PartialOrd + std::fmt::Debug + std::fmt::Display
{
	if array.len() == 1 {
        ()
    }
    let mut pos = 0;
    let last_idx = array.len() - 1;
    let mid = (last_idx) / 2;

    array.swap(mid, last_idx);
    for i in 0..last_idx {
        if array[i] < array[last_idx] {
            array.swap(i, pos);
            pos += 1;
        }
    }
    array.swap(pos, last_idx);

    if pos != 0 {
        sort(&mut array[0..pos]);
    }
    if pos != last_idx {
        sort(&mut array[pos+1..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}