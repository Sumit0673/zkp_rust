pub trait Group{
    type Element;

    fn operation(a: &Self::Element, b: &Self::Element) -> Self::Element;

    fn identity() -> Self::Element;

    fn inverse(a: &Self::Element) -> Self::Element;
    
}

pub struct ModuloGroup{
    pub modulus: i32
}


impl ModuloGroup{

    pub fn new(modulus: i32) -> Self{
        if modulus <=0{
            panic!("Can't Happen this")
        }
        ModuloGroup {modulus}
    }

    pub fn operation(&self, a: i32, b: i32) -> i32{
        (a + b).rem_euclid(self.modulus)
    }

    pub fn identity(&self) -> i32{
        0
    }
    
    pub fn inverse(&self, a:i32) -> i32{
        (-a).rem_euclid(self.modulus)
    }
    
}


fn main(){
    let group = ModuloGroup::new(10);

    let a = 11;
    let b = 15;

    let 
    sum = group.operation(a, b);
    println!("Modulus sum of {} & {} = {}", a, b, sum);

    let identity = group.identity();
    println!("Identity = {}", identity);

    let inverse_a = group.inverse(a);
    println!("Inverse of a mod 7 = {}", inverse_a);
}
