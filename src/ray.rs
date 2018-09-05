use vector::Vector;
use sphere::Sphere;

pub struct Ray {
    pub o: Vector,
    pub d: Vector,
}

impl Ray {
    pub fn new(o: Vector, d: Vector) -> Ray {
        Ray {
            o,
            d
        }
    }

    pub fn intersect(&self, t: &mut f32, id: &mut usize, spheres: &Vec<Sphere>) -> bool {
        let n = spheres.len();
        let mut d;//= 0.0;
        let mut inf = 1.0e20;
        *t = 1.0e20;
        for i in 0..n {
            d = spheres[i].intersect(self);
//            println!("{}", d);
            if d != 0.0 && d < *t {
                *t = d;
                *id = i;
            }
        }
        return *t < inf;
    }
}