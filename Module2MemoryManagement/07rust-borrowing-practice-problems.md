// Rust comments:
//  - Line comment: starts with // and goes to end of line
//  - Block comment: /* ... */ can span multiple lines

// ---------------- Problem #1 ----------------
// Borrow two Strings (no ownership taken) and return a new concatenated String.
fn concat_strings(s1: &String, s2: &String) -> String {
    // We borrow s1 and s2 and build a new owned String as the result.
    // Either of these would be fine:
    // format!("{}{}", s1, s2)
    // or:
    let mut out = String::with_capacity(s1.len() + s2.len());
    out.push_str(s1);
    out.push_str(s2);
    out
}

// ---------------- Problem #2 ----------------
// Borrow a String, clone it, modify the clone, and return the modified clone.
fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();      // make an owned copy; original is untouched
    cloned.push_str("World!");       // append to the clone only
    cloned
}

// ---------------- Problem #3 ----------------
// Write the sum of integers from low..=high into a caller-provided mutable slot.
fn sum(total: &mut i32, low: i32, high: i32) {
    // Make the function deterministic regardless of the caller's initial value.
    *total = 0;
    for i in low..=high {
        *total += i;                 // mutate through the &mut reference
    }
}

// ---------------- Run all three in one main ----------------
fn main() {
    // Problem #1 demo
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("Problem 1 -> {}", result); // Expect: Hello, World!

    // Problem #2 demo
    let base = String::from("Hello, ");
    let modified = clone_and_modify(&base);
    println!("Problem 2 -> Original: {}", base);     // Expect: Hello, 
    println!("Problem 2 -> Modified: {}", modified); // Expect: Hello, World!

    // Problem #3 demo
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Problem 3 -> Sum 0..=100 = {}", total); // Expect: 5050
}
