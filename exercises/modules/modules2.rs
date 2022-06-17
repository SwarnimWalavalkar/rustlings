// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {

    // TODO: Fix these use statements
    use self::fruits::PEAR as PEAR;
    use self::veggies::CUCUMBER as CUCUMBER;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruits::PEAR,
        delicious_snacks::veggies::CUCUMBER
    );
}
