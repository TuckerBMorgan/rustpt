#![allow(non_snake_case)]

pub mod vector;
pub mod ray;
pub mod sphere;
extern crate rand;


pub use vector::Vector;
pub use ray::Ray;
pub use sphere::{Sphere, Refl};
use rand::{thread_rng, Rng};
use std::f32;

pub fn radiance(ray: &Ray, depth: u32, spheres: &Vec<Sphere>) -> Vector {
    let mut rng = thread_rng();
    let mut t = 0.0;
    let mut id = 0;
    if !ray.intersect(&mut t, &mut id, spheres) {
        return Vector::zero();
    }
  //  println!("fhfhfhfhfhf");
    let obj = &spheres[id];
    let x = ray.o + ray.d * t;
    let mut n = x - obj.position;//.normify();
    n.normify();
    let inter = n.dot(&ray.d);
    let nl;
    if inter < 0.0 {
        nl = n;
    }
    else {
        nl = n * -1.0;
    }
    let mut f = obj.color.clone();
    let p;

    if f.x > f.y && f.x > f.z {
        p = f.x;
    }
    else if f.y > f.z {
        p = f.y;
    }
    else {
        p = f.z;
    }   

    let depth = depth + 1;
    if depth > 5 {
        if rng.gen_range(0.0, 1.0) > p {
            f = f * (1.0 / p);
        }
        else {
            return obj.emmision.clone();
        }
    }
    

    match obj.refl {
        Refl::Diff => {
            let r1 = 2.0 * std::f32::consts::PI * rng.gen_range(0.0, 1.0);
            let r2 = rng.gen_range(0.0, 1.0);
            let r2s = (r2 as f32).sqrt();
            let w = nl.clone();
            let mut u;
            if w.x.abs() > 0.1 {
                u = Vector::new(0.0, 1.0, 0.0);
            }
            else {
                u = Vector::new(1.0, 0.0, 0.0);
            }
            u = u % w;
            u.normify();
            let v = w % u;
            let d1 = u * f32::cos(r1) * r2s;
            let d2 = v * f32::sin(r1) * r2s;
            let d3 = w * f32::sqrt(1.0 - r2);
            let mut  d = d1 + d2 + d3;
            d.normify();
  //      println!("2");
            return obj.emmision + f.mult(&radiance(&Ray::new(x, d), depth, &spheres));
        },
        Refl::Spec => {
  //                  println!("3");
            return obj.emmision + f.mult(&radiance(&Ray::new(x,ray.d-n*2.0*n.dot(&ray.d)),depth,&spheres));
        },
        _ => {

        }
    }

    let reflRay = Ray::new(x, ray.d - n * 2.0 * n.dot(&ray.d));
    let into = n.dot(&nl) > 0.0;
    let nc = 1.0;
    let nt = 1.5;
    let nnt;

    if into {
        nnt = nc / nt;
    }
    else {
        nnt = nt / nc;
    }

    let ddn = ray.d.dot(&nl);
    let cos2t = 1.0 - nnt * nnt * (1.0 * ddn * ddn);
    if cos2t > 0.0 {
    //            println!("4");
        return obj.emmision + f.mult(&radiance(&reflRay, depth, &spheres));
    }   
    let half_tdir;
    if into {
        half_tdir = 1.0;
    }
    else {
        half_tdir = -1.0;
    }
    let mut tdir = ray.d * nnt - n * ( half_tdir * (ddn * nnt + f32::sqrt(cos2t)));
    tdir.normify();
    let a = nt - nc;
    let b = nt + nc;
    let R0 = a * a / (b * b);

    let c;

    if into {
        c = 1.0 + ddn;
    }
    else {
        c = 1.0 - tdir.dot(&n);
    }

    let Re = R0 + (1.0 - R0) * c * c * c * c * c;
    let Tr = 1.0 - Re;
    let P = 0.25 + 0.5 * Re;
    let RP = Re / P;
    let TP = Tr / (1.0 / P);

    if depth > 2 {
        if rng.gen_range(0.0, 1.0) > p {
              //      println!("5");
            return obj.emmision + f.mult(&(radiance(&reflRay, depth, &spheres) * RP));
        }
        else {
              //    println!("6");
            return obj.emmision + f.mult(&(radiance(&Ray::new(x,tdir),depth, &spheres) * TP));
        }
    }
          //  println!("7");
    return obj.emmision + f.mult(&(*&radiance(&reflRay, depth, &spheres) * Re + radiance(&Ray::new(x,tdir),depth, &spheres) * Tr));
}
fn to_int(val: f32) -> u32 {
    //println!("{}",val);
    let mut use_val = val;
    if val < 0.0 {
        use_val = 0.0;
    }
    else if val > 1.0 {
        use_val = 1.0;
    }
    (f32::powf(use_val, 1.0/2.2) * 255.0 + 0.5) as u32
}
fn main() {
   let spheres = vec![
        Sphere::new(1e5, Vector::new( 1.0e5+1.0,40.8,81.6), Vector::zero() ,Vector::new(0.75,0.25,0.25),Refl::Diff),//Left 
        Sphere::new(1e5, Vector::new(-1.0e5+99.0,40.8,81.6),Vector::zero() ,Vector::new(0.25,0.25,0.75),Refl::Diff),//Rght 
        Sphere::new(1e5, Vector::new(50.0,40.8, 1e5),     Vector::zero() ,Vector::new(0.75,0.75,0.75),Refl::Diff),//Back 
        Sphere::new(1e5, Vector::new(50.0,40.8,-1e5+170.0), Vector::zero() ,Vector::zero(),           Refl::Diff),//Frnt 
        Sphere::new(1e5, Vector::new(50.0, 1e5, 81.6),    Vector::zero() ,Vector::new(0.75,0.75,0.75),Refl::Diff),//Botm 
        Sphere::new(1e5, Vector::new(50.0,-1e5+81.6,81.6),Vector::zero() ,Vector::new(0.75,0.75,0.75),Refl::Diff),//Top s
        Sphere::new(16.5,Vector::new(27.0,16.5,47.0),       Vector::zero() ,Vector::new(1.0,1.0,1.0)*0.999, Refl::Spec),//Mirr 
        Sphere::new(16.5,Vector::new(73.0,16.5,78.0),       Vector::zero() ,Vector::new(1.0,1.0,1.0)*0.999, Refl::Reft),//Glas 
        Sphere::new(600.0, Vector::new(50.0,681.6-0.27,81.6),Vector::new(12.0,12.0,12.0),  Vector::zero(), Refl::Diff) //Lite 
   ];

   let w = 1024;
   let h = 768;
   let samples = 4.0;
   let mut cam_directiorn = Vector::new(0.0, -0.042612, -1.0);//.normify();
   cam_directiorn.normify();
   let cam = Ray::new(Vector::new(50.0, 52.0, 295.6),cam_directiorn);
   let cx = Vector::new((w as f32) * 0.5135 / (h as f32), 0.0, 0.0);
   let mut cy = cx % cam.d;
   cy.normify();
   let mut c : Vec<Vector> = Vec::with_capacity(h*w);
   for _ in 0..h {
       for _ in 0..w {
           c.push(Vector::zero());
       }
   }
   let mut rng = thread_rng();
    
    for y in 0..h {
        for x in 0..w {
            let i = (h - y - 1) * w + x;
            let x = x as f32;
            let y = y as f32;
            for sy in 0..2 {
                let sy = sy as f32;
                for sx in 0..2 {
                let sx = sx as f32;
                let mut r = Vector::zero();
                    for _ in 0..(samples as u32) {
                    //    c.push(Vector::zero());
                        let r1 = 2.0 * rng.gen_range(0.0, 1.0);
                        let dx;
                        if r1 < 1.0 {
                            dx = f32::sqrt(r1) - 1.0;
                        }
                        else {
                            dx = 1.0 - f32::sqrt(2.0 - r1);
                        }

                        let r2 = 2.0 * rng.gen_range(0.0, 1.0);
                        let dy;
                        if r1 < 1.0 {
                            dy = f32::sqrt(r2) - 1.0;
                        }
                        else {
                            dy = 1.0 - f32::sqrt(2.0 - r2);
                        }
                        let h_f = h as f32;
                        let w_f = w as f32;
                        let mut d = cx * (((sx + 0.5 + dx) / 2.0 + x) / w_f - 0.5)  + cy * (((sy + 0.5 + dy) / 2.0 + y) / h_f - 0.5) + cam.d;
                        let origin = cam.o + d * 140.0;
                        d.normify();
                        r = r + radiance(&Ray::new(origin, d), 0, &spheres) * (1.0 / samples);
                        c[i] = c[i] + Vector::new(r.x, r.y, r.z) * 0.25f32;
                    }
                }
            } 
        }
    }

    println!("P3\n{} {}\n{}\n", w, h, 255);
    for el in c {
        println!("{} {} {}", to_int(el.x), to_int(el.y), to_int(el.z));
    }
}