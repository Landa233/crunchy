fn split_into_maximal_sublists<T: Clone>(list: Vec<T>, number_of_chunks: usize) -> Vec<Vec<T>> {
    let quotient = list.len() / number_of_chunks;
    let remainder = list.len() % number_of_chunks;

    let mut chunk_sizes = vec![0; number_of_chunks];

    for i in 0..remainder {
        chunk_sizes[i] += 1;
    }

    for chunk in chunk_sizes.iter_mut() {
        *chunk += quotient;
    }

    let mut chunks = vec![];

    let mut list_iter = list.into_iter();
    for chunk_size in chunk_sizes {
        let chunk = list_iter.by_ref().take(chunk_size).collect();
        chunks.push(chunk);
    }

    chunks
}

fn main() {
    let mut counter = 1;
    let mut v = vec![counter];

    for _ in 0..10 {
        let res = split_into_maximal_sublists(v.clone(), 4);
        println!("{:?}", res);
        counter += 1;
        v.push(counter);
    }
}
