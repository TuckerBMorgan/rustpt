use vector::Vector;
use ray::Ray;

pub enum Refl {
    Diff,
    Spec,
    Reft
}

pub struct Sphere {
    pub rad: f32,
    pub position: Vector,
    pub emmision: Vector,
    pub color: Vector,
    pub refl: Refl
}


impl Sphere {
    pub fn new(rad: f32, position: Vector, emmision: Vector, color: Vector, refl: Refl) -> Sphere {
        Sphere {
            rad,
            position,
            emmision,
            color,
            refl
        }
    }

    pub fn intersect(&self, r: &Ray) -> f32 {
        let op = self.position - r.o;
        let eps = 1e-4;
        let b = op.dot(&r.d);
        let mut det = b * b - op.dot(&op) + self.rad * self.rad;
        if det < 0.0 {
    //        println!("This");
            return 0.0;
        }
        det = det.sqrt();
        let t1 = b - det;
        let t2 = b + det;

        if t1 > eps {
            return t1;
        }

        if t2 > eps {
            return t2;
        }
//            println!("that");
        return 0.0;
    }
}

