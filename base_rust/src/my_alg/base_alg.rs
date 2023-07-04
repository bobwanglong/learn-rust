
// pub fn quickSort(q: &mut Vec<i64>,left:u32,right:u32) {
//     if left <= right{
//         return;
//     }

//     let x = q[left+right>>1];
//     let mut i = left-1;
//     let mut j = right+1;

//     while i<j {
//     //   while  {
//     //       i+=1;
//     //       if q[i]>=x{
//     //         break;
//     //       }
//     //   }
//     //   while  {
//     //       j-=1
//     //       if q[j]<=x{
//     //         break;
//     //       };
//     //   };
//       if i<j{
//         // q[i],q[j] = q[j],q[i];
    
//       }
//     }
//     quickSort(q, left, i);
//     quickSort(q, j+1,right) 

// }

fn quick_sort(q: &mut [i32], l: i32, r: i32) {
    if l >= r {
        return;
    }
    let x = q[((l + r) / 2)as usize];
    let mut i = l - 1;
    let mut j = r + 1;
    while i < j {
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
    quick_sort(q, l, j);
    quick_sort(q, j + 1, r);
}
#[test]
fn test_quick_sort() {
    let mut arr = [3, 2, 1];
    quick_sort(&mut arr, 0, 2);
    assert_eq!(arr, [1, 2, 3]);
}