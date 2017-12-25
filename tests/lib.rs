
#[allow(unused_imports)]

#[macro_use(EnumTools)]
extern crate enum_tools;

#[cfg(test)]
mod tests {

    #[derive(EnumTools)]
    enum Test {
        A(i32),
        B(i32, i32),
        C,
        D{x: i32, y: i32},
    }

    #[test]
    fn test_a() {
        let t = Test::A(0);
        assert!(t.is_A());
        assert!(!t.is_B());
        assert_eq!("A", t.name());

        let a = t.unwrap_A();
        assert_eq!(a, 0);
    }

    #[test]
    fn test_b() {
        let t = Test::B(0, 1);
        assert!(t.is_B());
        assert!(!t.is_A());
        assert_eq!("B", t.name());

        let (a, b) = t.unwrap_B();
        assert_eq!(a, 0);
        assert_eq!(b, 1);
    }

    #[test]
    fn test_c() {
        let t = Test::C;
        assert!(t.is_C());
        assert_eq!("C", t.name());
    }

    #[test]
    fn test_d() {
        let t = Test::D{x: 0, y: 1};
        assert!(t.is_D());
        assert_eq!("D", t.name());

        let (x, y) = t.unwrap_D();
        assert_eq!(x, 0);
        assert_eq!(y, 1);
    }


    #[test]
    fn test_e() {
        let t = Test::A(0);

        let _: &i32 = t.unwrap_A_ref();
    }

    #[test]
    fn test_f() {
        let t = Test::D{x: 0, y: 1};

        let (_, _): (&i32, &i32)  = t.unwrap_D_ref();
    }


}
