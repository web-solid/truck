use truck_modeling::*;
use truck_stepio::out::*;
use truck_topology::compress::*;

macro_rules! dir ( () => { concat!(env!("CARGO_MANIFEST_DIR"), "/../resources/shape/") });

const SOLID_JSONS: [&str; 3] = [
    concat!(dir!(), "bottle.json"),
    concat!(dir!(), "punched-cube.json"),
    concat!(dir!(), "torus-punched-cube.json"),
];

#[test]
fn parse_solid() {
    for json_file in SOLID_JSONS.iter() {
        let json = std::fs::read(json_file).unwrap();
        let solid: CompressedSolid<Point3, Curve, Surface> =
            serde_json::from_reader(json.as_slice()).unwrap();
        let step_string =
            CompleteStepDisplay::new(StepModel::new(&solid), Default::default()).to_string();
        ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
            panic!(
                "failed to parse step from {}\n[Error Message]\n{}[STEP file]\n{}",
                json_file, e, step_string
            )
        });
    }
}

#[test]
fn parse_shell() {
    for json_file in SOLID_JSONS.iter() {
        let json = std::fs::read(json_file).unwrap();
        let mut solid: CompressedSolid<Point3, Curve, Surface> =
            serde_json::from_reader(json.as_slice()).unwrap();
        let shell = solid.boundaries.pop().unwrap();
        let step_string =
            CompleteStepDisplay::new(StepModel::new(&shell), Default::default()).to_string();
        ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
            panic!(
                "failed to parse step from {}\n[Error Message]\n{}[STEP file]\n{}",
                json_file, e, step_string
            )
        });
    }
}
