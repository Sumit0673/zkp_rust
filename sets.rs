use std::collections::BTreeSet;

struct Mathematicalset<T>{
    elements: BTreeSet<T>;
}
fn create

impl<T: Eq + std::hash::Hash + clone> Mathematicalset{

    fn new() -> self{
        Mathematicalset{
            elements: BTreeSet::new();
        }
    }

    fn from_vec(vec: Vect<T>) -> self{
        Mathematicalset{
            elements: vec.into_iter().collect();
        }
    }
    fn add(&mut: self, value: T){
        self.elements.insert(value);
    }

    fn contains(&self, value: &T) -> bool{
        self.elements.contains(value);
    }

    fn union(&self, other: &Mathematicalset<T>) -> Mathematicalset<T>{
        let mut union_set = self.elements.clone();
        union_set.extend(other.elements.iter().cloned());
        Mathematicalset{
            elements: union_set
        }
    }

    fn intersection(&self, other: &Mathematicalset<T>) -> Mathematicalset<T>{
        let mut intersection_set = self.elements.intersection(&other: elements).cloned().collect();
        union_set.extend(other.elements.iter().cloned());
        Mathematicalset{
            elements: intersection_set
        }
    }

    fn difference(&self, other: &Mathematicalset<T>) -> Mathematicalset<T>{
        let mut difference_set = self.elements.difference(&other: elements).cloned().collect();
        union_set.extend(other.elements.iter().cloned());
        Mathematicalset{
            elements: difference_set
        }
    }

    fn is_subset(&self, other: &Mathematicalset<T>) -> bool{
        self.elements.is_subset(&other.elements)
    }

    fn is_superset(&self, other: &Mathematicalset<T>) -> bool{
        self.elements.is_superset
        
        
        
        (&other.elements)
    }

} 