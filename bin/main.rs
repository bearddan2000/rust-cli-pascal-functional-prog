fn row(k :i32, i :i32, c :i32) {
  if k > i {
    return;
  }
  print!( "{}, ", c);
  let a = c * (i-k)/(k+1);
  return row(k+1, i, a);
}
fn col(i :i32, n :i32) {
  if i > n {
    return;
  }
  row(0, i, 1);
  println!("");
  return col(i+1, n);
}

fn main() {
  let input = 10;
  println!( "[INPUT] {}", input);
  println!( "[OUTPUT] ");
  col(0, input);
}
