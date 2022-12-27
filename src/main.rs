use std::fmt;

fn main(){
    // Integer addition
    println!("1+2={}", 1u32+2);

    // Integer subtraction
    println!("1-2={}", 1i32-2);

    println!("1+2={}", 1u32+2);
//    println!("1-2={}", 1u32-2);
    // type error
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("One million is wrriten as {}", 1_000_000i32);
}

fn format() {
    println!("Hello, world!");

    println!("{} days", 31);

    println!("{0}, this is {1}. {1} , this is {0}", "Aliece", "Bob");

    // :b is binary mode
    // :o is octal (8進数)
    // :x is hexadecital (16)
    println!("{} of {:b} people know binary, the other half doesn't", 1,2);

    // 右・左寄せ、0埋め
    println!("{number:>width$}", number=1, width=6);
    println!("{number:<width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);
    println!("{number:0<width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    struct Structure(i32);

    // derive attribute automatically create the implementation
    // use :? format
    println!("This struct `{:?}` won't print...", Structure(3));

    #[derive(Debug)]
    struct Deep(Structure);
    println!("Deep print ?={:?}", Deep(Structure(5)));
    println!("Deep print #={:#?}", Deep(Structure(5)));

    println!("{1:?} {0:?} is the {actor:?}", "Slater", "Christian", actor="actor's");

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    let name ="Peter";
    let age = 27;
    let peter = Person{name, age};

    println!("{:#?}", peter);

    struct FmtStruct(i32);
    impl fmt::Display for FmtStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "{}", self.0 )
        }
    }
    println!("Format Structure {}", FmtStruct(49));

    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement Display
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D{
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "(x: {}, y: {})", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug:{:?}", minmax);

    let point = Point2D{x:3.3, y:7.2};
    println!("Compare structures:");
    println!("Display: {}", point);
    println!("Debug:{:?}", point);

    #[derive(Debug)]
    struct Complex{
        real: f32,
        imag: f32,
    }
    impl fmt::Display for Complex{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f,"{} + {}i", self.real, self.imag)
        }
    }
    let complex = Complex{real:3.3, imag:7.2};
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    // Vec
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            // open bracket
            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", ")?;}
                write!(f, "{0}: {1}", count,v)?;
            }

            // close bracket
            write!(f, "]")
        }
    }

    let v = List(vec![1,2,3,4]);
    println!("{}", v);

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "RGB ( {red}, {green}, {blue} ) 0x{red:02X}{green:02X}{blue:02X}", red=self.red, green=self.green, blue=self.blue)
        }
    }
    let colod_a = Color{red:128, green:255, blue:90};
    println!("Debug: {:?}", colod_a);
    println!("Display: {}", colod_a);

    let colod_b = Color{red:0, green:3, blue:254};
    println!("Debug: {:?}", colod_b);
    println!("Display: {}", colod_b);

    let colod_c = Color{red:0, green:0, blue:0};
    println!("Debug: {:?}", colod_c);
    println!("Display: {}", colod_c);

}
