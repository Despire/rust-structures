use strings::strtok;

#[test]
fn test_strtok() {
    let want = [
        "some",
        "long",
        "string",
        "separated",
        "by",
        "single",
        "white",
        "spaces",
    ];

    let mut some_string = "some long string separated by single white spaces";

    let delim = ' ';
    let mut prev = strtok(&mut some_string, delim);

    assert_eq!(prev, want[0]);

    let mut r = 1;
    loop {
        let curr = strtok(&mut some_string, delim);
        if curr == prev {
            break;
        }

        assert_eq!(curr, want[r]);
        prev = curr;
        r += 1;
    }

    assert_eq!(prev, want[want.len() - 1]);
}
