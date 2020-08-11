use specs::prelude::*;
use roguelike_common::*;
use super::components::*;
use super::map::*;
//use num_integer::Roots;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = ( ReadExpect<'a, Map>,
                        WriteStorage<'a, FieldOfView>, 
                        ReadStorage<'a, Position>);

    fn run(&mut self, (map, mut fov, pos): Self::SystemData) {
        for (fov, pos) in (&mut fov, &pos).join() {
            if fov.dirty {
                fov.dirty = false;
                fov.visible_tiles.clear();
                fov.visible_tiles = get_fov(pos.x, pos.y, fov.range);
                fov.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
            }
        }
    }
}

fn get_fov(x: i32, y: i32, r: i32) -> Vec<Position> {
    let mut v = vec!();
    for i in r*-1..r+1 {
        let mut n = (((r*r - i*i) as f64).sqrt()) as i32;
        n = match i {
            -1 | 1 => n+1,
            _ => n,
        };
        n = if n == 0 { 1 } else { n };
        for j in n*-1..n+1 {
            v.push(Position { x: x+i, y: y+j })
        }
    }
    v
}

//Calculate angle at P1 = 0,0
//
//private double calculateAngle(double P1X, double P1Y, double P2X, double P2Y,
//        double P3X, double P3Y){
//
//    double numerator = P2Y*(P1X-P3X) + P1Y*(P3X-P2X) + P3Y*(P2X-P1X);
//    double denominator = (P2X-P1X)*(P1X-P3X) + (P2Y-P1Y)*(P1Y-P3Y);
//    double ratio = numerator/denominator;
//
//    double angleRad = Math.Atan(ratio);
//    double angleDeg = (angleRad*180)/Math.PI;
//
//    if(angleDeg<0){
//        angleDeg = 180+angleDeg;
//    }
//
//    return angleDeg;
//}

