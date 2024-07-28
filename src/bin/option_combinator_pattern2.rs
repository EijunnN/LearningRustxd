// Topic: Option combinators
//
// Requirements:
// * Use combinators as described in the functions:
//   part_1, part_2, and part_3
//
// Notes:
// * Run `cargo test --bin a23` to check your program.
// * Only edit the part_1, part_2, and part_3 functions.

fn part_1() -> bool {
    // We are checking whether or not this particular user
    let user = maybe_access("admin");
    user.is_some()

    // has an access level. The "admin" user does have
    // an access level.
    // Note: Use is_some or is_none.
}

fn part_2() -> Option<Access> {
    // "Root" is equivalent to Access::Admin, but it is
    // not listed in the maybe_access function.
    // Note: Use or_else and root().
    let user = maybe_access("root");
    user.or_else(|| root())

}

fn part_3() -> Access {
    // "Alice" is not a listed user, so she will be a guest.
    // Note: Use unwrap_or_else.
    let user = maybe_access("Alice");
    user.unwrap_or_else(|| Access::Guest )
}

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Admin,
    User,
    Guest,
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None,
    }
}

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_part_1() {
        assert_eq!(part_1(), true, "Admins have an access level");
    }

    #[test]
    fn check_part_2() {
        assert_eq!(
            part_2(),
            Some(Access::Admin),
            "Root users have Admin access"
        );
    }

    #[test]
    fn check_part_3() {
        assert_eq!(part_3(), Access::Guest, "Alice is a guest");
    }
}
