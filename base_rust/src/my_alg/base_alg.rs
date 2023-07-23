use std::vec;


pub fn quickSort(q: &mut Vec<i64>,left:i32,right:i32) {
    if left >= right{
        return;
    }

    let x = q[((left+right)>>1) as usize];
    let mut i = left-1;
    let mut j = right+1;

    while i<j {
        i += 1;
        while q[i as usize] < x {
            i += 1;
        }
        j -= 1;
        while q[j as usize] > x {
            j -= 1;
        }
        if i < j {
            q.swap(i as usize, j as usize);
        }
    }
    quickSort(q, left, i);
    quickSort(q, j+1,right) 

}

fn quick_sort(q: &mut [i32], l: i32, r: i32) {
    if l >= r {
        return;
    }
    let x = q[((l + r) / 2)as usize];
    let mut i = l ;
    let mut j = r ;
    while i < j {

        while q[i as usize] < x {
            i += 1;
        }
        while q[j as usize] > x {
            j -= 1;
        }
        if i < j {
            q.swap(i as usize, j as usize);
        }
    }
    quick_sort(q, l, j);
    quick_sort(q, j + 1, r);
}
#[test]
fn test_quick_sort1() {
    let mut arr = [3, 2, 1];
    quick_sort(&mut arr, 0, 2);
    assert_eq!(arr, [1, 2, 3]);
}
#[test]
fn test_quick_sort2() {
    let mut arr:Vec<i64> = vec![3, 2, 1];
    quickSort(&mut arr, 0, 2);
    assert_eq!(arr, [1, 2, 3]);
}