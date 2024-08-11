///Just contains tests for my bubble sort algorithm implementations.
fn main() {
    let mut vec = vec![64, 34, 25, 12, 22, 11, 90];
    let mut vec2 = vec![64, 34, 25, 12, 22, 11, 90];

    let sorted_vec = bubble_sort_recursion(&mut vec);
    let normal_sorted_vec = bubble_sort(&mut vec2);

    ///This is the first test using the recursive function
    println!("Recursive Technique: {:?}", sorted_vec);

    ///This is the second test, using the actual bubble sort technique.
    println!("Normal Technique: {:?}", normal_sorted_vec);
}

///first implementation. Technically not correct, but gets the job done.
fn bubble_sort_recursion(a: &mut Vec<i32>) -> &mut Vec<i32> {

    let mut swapped = false;

    for int in 1..a.len() {
        if a[int - 1] > a[int] {
            let temp_val = a[int - 1];

            a[int - 1] = a[int];
            a[int] = temp_val;

            swapped = true;
        }
    }

    if swapped {
        bubble_sort_recursion(a);
    }

    a

}

//A more traditional and correct version of bubble sort. Works like a charm!
fn bubble_sort(a : &mut Vec<i32>) -> &mut Vec<i32> {
    for int in 0..a.len() {

        let mut swapped = false;

        for i in 0.. a.len() - int - 1 {
            if a[i] > a[i + 1] {
                let temp_val = a[i];

                a[i] = a[i + 1];
                a[i + 1] = temp_val;

                swapped = true;
            }
        }

        if !swapped {
            break
        }

    }

    a

}