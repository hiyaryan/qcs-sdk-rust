//! This integration test implements the [Parametric Compilation example from pyQuil][example].
//! [example]: https://pyquil-docs.rigetti.com/en/stable/basics.html?highlight=parametric#parametric-compilation

use std::f64::consts::PI;

use qcs::Executable;

const PROGRAM: &str = r#"
DECLARE ro BIT
DECLARE theta REAL

RX(pi / 2) 0
RZ(theta) 0
RX(-pi / 2) 0

MEASURE 0 ro[0]
"#;

#[tokio::test]
async fn happy() {
    let mut exe = Executable::from_quil(PROGRAM);
    let mut parametric_measurements = Vec::with_capacity(200);

    let step = 2.0 * PI / 200.0;

    for i in 0..=200 {
        let theta = step * f64::from(i);
        let result = exe
            .with_parameter("theta", 0, theta)
            .execute_on_qvm()
            .await
            .expect("Failed to execute");
        parametric_measurements.append(&mut result.into_i8().unwrap()[0])
    }

    for measurement in parametric_measurements {
        if measurement == 1 {
            // We didn't run with all 0 so parametrization worked!
            return;
        }
    }
    panic!("Results were all 0, parametrization must not have worked!");
}

const ARITHMETIC: &str = r#"
DECLARE ro REAL[2]
DECLARE theta REAL

MOVE ro[0] theta
MOVE ro[1] theta
SUB ro[1] 0.5
"#;

#[tokio::test]
async fn arithmetic() {
    let mut exe = Executable::from_quil(ARITHMETIC);

    for i in 0..=5 {
        let theta = f64::from(i);
        let result = exe
            .with_parameter("theta", 0, theta)
            .execute_on_qvm()
            .await
            .expect("Failed to execute")
            .into_f64()
            .unwrap();
        assert_eq!(result[0][0], theta);
        assert_eq!(result[0][1], theta - 0.5);
    }
}

const ARITHMETIC_GATE: &str = r#"
DECLARE ro BIT
DECLARE theta REAL

RX(theta / 2) 0

MEASURE 0 ro[0]
"#;

#[tokio::test]
async fn arithmetic_gate() {
    let mut exe = Executable::from_quil(ARITHMETIC_GATE);

    let result = exe
        .with_parameter("theta", 0, PI * 2.0)
        .execute_on_qvm()
        .await
        .expect("Failed to execute")
        .into_i8()
        .unwrap();
    assert_eq!(result[0][0], 1);
}