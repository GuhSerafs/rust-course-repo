pub struct SimpleCircle {
    center: (f64, f64),
    radius: f64,
}

impl SimpleCircle {
    pub fn new(center: (f64, f64), radius: f64) -> Self {
        SimpleCircle {
            center,
            radius
        }
    }

    pub fn area(&self) -> f64 {
        3.14159*self.radius*self.radius
    }

    pub fn distance_from_origin(&self) -> f64 {
        (self.center.0*self.center.0 + self.center.1*self.center.1).sqrt()
        
    }
}