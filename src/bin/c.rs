#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            line.pop();
            line
        }
    )
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let iter = line.split_whitespace();
            iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
        }
    )
}

fn main() {
    let n = parse_line!(usize);
    let x = vec![0; n];
    let mut s:Vec<Vec<i32>> = Vec::new();
    s.push(x.clone());
    s.push(x.clone());
    let mut t:Vec<Vec<i32>> = Vec::new();
    t.push(x.clone());
    t.push(x.clone());
    for i in 0..n{
        let s_num = parse_vec!(char);
        for j in 0..n{
            if s_num[j] == '#'{
                s[i][j] = 1;
            }
        }
    }
    for i in 0..n{
        let t_num = parse_vec!(char);
        for j in 0..n{
            if t_num[j] == '#'{
                t[i][j] = 1;
            }
        }
    }
    
}
