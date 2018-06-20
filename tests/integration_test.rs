// Still want to test
// Non-numeric input
// Empty Input

extern crate intl_pluralrules;

use intl_pluralrules::*;

#[test]
fn int_0() {
    assert_eq!(
        PluralOperands {
            n: 0_f64,
            i: 1,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            int: 0,
            dec: 0,
        },
        intl_pluralrules::PluralOperands::new("0")
    );
}

#[test]
fn int_2() {
    assert_eq!(
        PluralOperands {
            n: 2_f64,
            i: 1,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            int: 2,
            dec: 0,
        },
        intl_pluralrules::PluralOperands::new(2)
    );
}

#[test]
fn int_57() {
    assert_eq!(
        PluralOperands {
            n: 57_f64,
            i: 2,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            int: 57,
            dec: 0,
        },
        intl_pluralrules::PluralOperands::new(57)
    );
}

#[test]
fn int_987() {
    assert_eq!(
        PluralOperands {
            n: 987_f64,
            i: 3,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            int: 987,
            dec: 0,
        },
        intl_pluralrules::PluralOperands::new(987)
    );
}

#[test]
fn int_1234567() {
    assert_eq!(
        PluralOperands {
            n: 1234567_f64,
            i: 7,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            int: 1234567,
            dec: 0,
        },
        intl_pluralrules::PluralOperands::new(1234567)
    );
}

#[test]
fn int_neg10() {
    assert_eq!(
        PluralOperands {
            n: 10_f64,
            i: 2,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            int: 10,
            dec: 0,
        },
        intl_pluralrules::PluralOperands::new(-10)
    );
}

#[test]
fn int_neg1000000() {
    assert_eq!(
        PluralOperands {
            n: 1000000_f64,
            i: 7,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            int: 1000000,
            dec: 0,
        },
        intl_pluralrules::PluralOperands::new(-1000000)
    );
}

#[test]
fn int_023() {
    assert_eq!(
        PluralOperands {
            n: 0.23_f64,
            i: 1,
            v: 2,
            w: 2,
            f: 23,
            t: 23,
            int: 0,
            dec: 23,
        },
        intl_pluralrules::PluralOperands::new(0.23)
    );
}

#[test] // Notice Assert *Not Equal*
fn int_0230() {
    assert_ne!(
        PluralOperands {
            n: 0.23_f64,
            i: 1,
            v: 3,
            w: 2,
            f: 230,
            t: 23,
            int: 0,
            dec: 23,
        },
        intl_pluralrules::PluralOperands::new(0.230)
    );
}

#[test] 
fn int_0230string() {
    assert_eq!(
        PluralOperands {
            n: 0.23_f64,
            i: 1,
            v: 3,
            w: 2,
            f: 230,
            t: 23,
            int: 0,
            dec: 230,
        },
        intl_pluralrules::PluralOperands::new("0.230".to_string())
    );
}

#[test] 
fn int_0203000string() {
    assert_eq!(
        PluralOperands {
            n: 0.0203000_f64,
            i: 1,
            v: 7,
            w: 4,
            f: 203000,
            t: 203,
            int: 0,
            dec: 203000,
        },
        intl_pluralrules::PluralOperands::new("0.0203000".to_string())
    );
}

#[test]
fn int_123dot45() {
    assert_eq!(
        PluralOperands {
            n: 123.45_f64,
            i: 3,
            v: 2,
            w: 2,
            f: 45,
            t: 45,
            int: 123,
            dec: 45,
        },
        intl_pluralrules::PluralOperands::new(123.45)
    );
}

#[test]
fn int_neg1234dot567() {
    assert_eq!(
        PluralOperands {
            n: 1234.567_f64,
            i: 4,
            v: 3,
            w: 3,
            f: 567,
            t: 567,
            int: 1234,
            dec: 567,
        },
        intl_pluralrules::PluralOperands::new(-1234.567)
    );
}