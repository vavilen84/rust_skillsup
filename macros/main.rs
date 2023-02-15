macro_rules! foo {
    (d => $e:expr) => (foo!(y => $e));
    (x => $e:expr) => (println!("mode X: {}", $e));
    (y => $e:expr) => (println!("mode Y: {}", $e));
}

fn main() {
    foo!(d => 3);
}
