 use std::cmp;
//////////////////////////////////////////////////////////////////////////////////////////////////
// Input of a sorting algorithm need not always be integers.
//
// It could very well be the case that we want to sort a list of consumers
// each with a unique id.
//
// Any algorithm hence written to be robust should follow the generic programming paradigm.
//
// We define a generic type placeholder 'sortable' with the suitable traits for comparison.
//
// ///////////////////////////////////////////////////////////////////////////////////////////////
pub fn insertion_sort<Sortable>(arr: &mut [Sortable]) where Sortable: cmp::PartialOrd + Copy
{
    /*
     A function that performs inplace sorting on a given array of the 
     generic type 'sortable'.
    */
    for j in 1..arr.len() {
        let key = arr[j];
        let mut i = j-1;
        
        while arr[i] > key {
            arr[i+1] = arr[i];
            if i == 0 {
                break;
            }
            i -= 1;
        }

        if i == 0 && arr[0] > key {
            arr[0] = key;
        } else {
            arr[i+1] = key;
        }

    }
}
