pub trait Group{
    type elements;

    fn operation(a: &self::elements, b: &self::elements) -> self::elements;

    fn identity() -> self::elements;

    fn inverse(a: &self::elements) -> self::elements;
    //new
    
}

