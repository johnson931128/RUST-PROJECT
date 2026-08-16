use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Intersection {
    x: f32,
    y: f32,
}

struct Vehicle {
    x: f32,
    y: f32,
    speed: f32,
}


#[macroquad::main("Urban Logistics Simulator")]
async fn main() {
    let intersections = [
        Intersection { x: 150.0, y: 150.0 },
        Intersection { x: 350.0, y: 150.0 },
        Intersection { x: 550.0, y: 150.0 },
        Intersection { x: 150.0, y: 350.0 },
        Intersection { x: 350.0, y: 350.0 },
        Intersection { x: 550.0, y: 350.0 },
        Intersection { x: 150.0, y: 550.0 },
        Intersection { x: 350.0, y: 550.0 },
        Intersection { x: 550.0, y: 550.0 },
    ];

    let roads = [
        (0, 1),
        (1, 2),
        (3, 4),
        (4, 5),
        (6, 7),
        (7, 8),
        (0, 3),
        (3, 6),
        (1, 4),
        (4, 7),
        (2, 5),
        (5, 8),
    ];

    let mut vehicle = Vehicle {
        x: intersections[0].x,
        y: intersections[0].y,
        speed: 100.0,
    };

    loop {
        clear_background(LIGHTGRAY);

        for &(start, end) in &roads {
            let a = intersections[start];
            let b = intersections[end];

            draw_line(a.x, a.y, b.x, b.y, 8.0, DARKGRAY);
        }

        for intersection in &intersections {
            draw_circle(intersection.x, intersection.y, 12.0, BLUE);
        }

        let dt = get_frame_time();
        
        let target_x = intersections[1].x;
        if vehicle.x < target_x {
            vehicle.x += vehicle.speed * dt;

            if vehicle.x > target_x{
                vehicle.x = target_x;
            }
        }
        draw_circle(vehicle.x, vehicle.y, 8.0, RED);

        next_frame().await;
    }
}