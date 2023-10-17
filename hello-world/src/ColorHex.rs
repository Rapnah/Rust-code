use std:: fmt::{self,Formatter,Display};

#[derive(Debug)]
struct Color
{
    red: u32,
    blue: u32,
    green: u32,
    //RGB: u8,
}
impl Display for Color
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let (r,g,b) = (self.red,self.green,self.blue);
        let rgb = (r*65536)+(g*256)+b;
        write!(f,"RGB ({}, {}, {}) 0x{:X}",
            self.red,self.green,self.blue,&rgb)
    }
}

fn main()
{
    for color in 
    [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] 
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}
