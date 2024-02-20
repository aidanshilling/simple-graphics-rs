use clap::Parser;
// Setup for parsing command line args using the clap.rs crate
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None, disable_version_flag = true)]
pub struct Args {
    /// Input ps file to use
    #[arg(short = 'f')]
    pub file: String,

    /// Placeholder
    #[arg(short = 'F', allow_negative_numbers = true, default_value_t = 0.6)]
    pub F: f64,

    /// Placeholder
    #[arg(short = 'B', allow_negative_numbers = true, default_value_t = -0.6)]
    pub B: f64,

    /// Integer lower bound in the x dimension of the viewport screen coordinates
    #[arg(short = 'j', allow_negative_numbers = true, default_value_t = 0)]
    pub j: i32,

    /// Integer lower bound in the y dimension of the viewport screen coordinates
    #[arg(short = 'k', allow_negative_numbers = true, default_value_t = 0)]
    pub k: i32,

    /// Integer upper bound in the x dimension of the viewport screen coordinates
    #[arg(short = 'o', allow_negative_numbers = true, default_value_t = 500)]
    pub o: i32,

    /// Integer upper bound in the y dimension of the viewport screen coordinates
    #[arg(short = 'p', allow_negative_numbers = true, default_value_t = 500)]
    pub p: i32,

    /// floating point x of Projection Reference Point (PRP) in VRC coordinatesFloating point x of Pr
    #[arg(short = 'x', allow_negative_numbers = true, default_value_t = 0.0)]
    pub x: f64,

    /// floating point y of Projection Reference Point (PRP) in VRC coordinates
    #[arg(short = 'y', allow_negative_numbers = true, default_value_t = 0.0)]
    pub y: f64,

    /// floating point z of Projection Reference Point (PRP) in VRC coordinates
    #[arg(short = 'z', allow_negative_numbers = true, default_value_t = 1.0)]
    pub z: f64,

    /// floating point x of View Reference Point (VRP) in world coordinates
    #[arg(short = 'X', allow_negative_numbers = true, default_value_t = 0.0)]
    pub X: f64,

    /// floating point y of View Reference Point (VRP) in world coordinates
    #[arg(short = 'Y', allow_negative_numbers = true, default_value_t = 0.0)]
    pub Y: f64,

    /// floating point z of View Reference Point (VRP) in world coordinates
    #[arg(short = 'Z', allow_negative_numbers = true, default_value_t = 0.0)]
    pub Z: f64,

    /// floating point x of View Plane Normal vector (VPN) in world coordinates
    #[arg(short = 'q', allow_negative_numbers = true, default_value_t = 0.0)]
    pub q: f64,

    /// floating point y of View Plane Normal vector (VPN) in world coordinates
    #[arg(short = 'r', allow_negative_numbers = true, default_value_t = 0.0)]
    pub r: f64,

    /// floating point z of View Plane Normal vector (VPN) in world coordinates
    #[arg(short = 'w', allow_negative_numbers = true, default_value_t = -1.0)]
    pub w: f64,

    /// floating point x of View Up Vector (VUP) in world coordinates
    #[arg(short = 'Q', allow_negative_numbers = true, default_value_t = 0.0)]
    pub Q: f64,

    /// floating point y of View Up Vector (VUP) in world coordinates
    #[arg(short = 'R', allow_negative_numbers = true, default_value_t = 1.0)]
    pub R: f64,

    /// floating point z of View Up Vector (VUP) in world coordinates
    #[arg(short = 'W', allow_negative_numbers = true, default_value_t = 0.0)]
    pub W: f64,

    /// floating point u min of the VRC window in VRC coordinates
    #[arg(short = 'u', allow_negative_numbers = true, default_value_t = -0.7)]
    pub u: f64,

    /// floating point v min of the VRC window in VRC coordinates
    #[arg(short = 'v', allow_negative_numbers = true, default_value_t = -0.7)]
    pub v: f64,

    /// floating point u max of the VRC window in VRC coordinates
    #[arg(short = 'U', allow_negative_numbers = true, default_value_t = 0.7)]
    pub U: f64,

    /// floating point v max of the VRC window in VRC coordinates
    #[arg(short = 'V', allow_negative_numbers = true, default_value_t = 0.7)]
    pub V: f64,

    /// Use parallel projection. If this flag is not present, use perspective projection
    #[arg(short = 'P', action)]
    pub P: bool,
}
