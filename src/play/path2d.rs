use super::*;

pub mod color {
    use super::*;

    pub const BALL: Color = Color::CYAN;
    pub const PIN: Color = Color::RED;
    pub const BOUNDS: Color = Color::RED;

    pub const FREE: Color = Color::WHITE;
    pub const KICK_GRAB: Color = Color::ORANGE;
    pub const SHOOT: Color = Color::YELLOW;
    pub const AIM: Color = Color::RED;
    pub const DRIBBLE: Color = Color::GREEN;
}

pub fn bounds(root: &Data) -> Path2d {
    Path2d {
        color: color::BOUNDS,
        points: [
            rect_points(root.constant.player_bounds),
            rect_points(root.constant.ball_bounds),
        ]
        .concat(),
        thickness: 1.,
        line_breaks: vec![9],
    }
}

pub fn player(root: &Data) -> Path2d {
    Path2d {
        color: color::FREE,
        points: [
            circle_points(root.constant.player_radius, 8),
            vec![Vec2::ZERO, Vec2::X * root.constant.player_radius],
        ]
        .concat(),
        thickness: 1.,
        line_breaks: vec![9],
    }
}
pub fn aim_arrow(root: &Data) -> Path2d {
    Path2d {
        color: color::AIM,
        points: vec![-Vec2::X * root.constant.player_radius, -Vec2::X * 4.],
        thickness: 1.,
        line_breaks: vec![],
    }
}
pub fn ball(root: &Data) -> Path2d {
    Path2d {
        color: color::BALL,
        points: circle_points(root.constant.ball_radius, 8),
        thickness: 1.,
        line_breaks: vec![],
    }
}
pub fn pin(root: &Data) -> Path2d {
    Path2d {
        color: color::PIN,
        points: circle_points(root.constant.pin_radius, 10),
        thickness: 1.,
        line_breaks: vec![],
    }
}
