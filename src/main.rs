type ISize = usize;
type Fibo = u128;

fn main() {
    println!("\n");

    let mut my_index: (ISize, ISize) = (0, 1);
    let mut fibo_vec: Vec<Fibo> = vec![0, 1];

    for _item in 0..=150 {
        next_fibo(&mut fibo_vec, &mut my_index);
    }

    for item in fibo_vec {
        println!(" -> {item}");
    }

    println!("\n The End ... (0.0.3)\n");
}

fn next_fibo(vec: &mut Vec<Fibo>, idx: &mut (ISize, ISize)) {
    let result = vec[idx.0] + vec[idx.1];
    vec.push(result);
    idx.0 += 1;
    idx.1 += 1;
}
