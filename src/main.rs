fn main() {
    println!("{:?}",find_prime_factors(18));
}

fn find_prime_factors(mut n:u128)->Vec<u128>{
    let mut p=2;
    let mut res=Vec::new();
    while n>=p*p{
        if n%p==0{
            res.push(p);
            n/=p;
        }
        else{
            p+=1;
        }
    }
    res.push(n);
    res
}
