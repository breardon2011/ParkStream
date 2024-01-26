struct Turbine {
    current_rpm: f64,
    maximum_rpm: f64,
    throttle_position: f64,
    tau: f64,
    k: f64,
}

impl Turbine {
    fn new(tau: f64, k: f64) -> Turbine {
        Turbine {
            current_rpm: 0.0,
            throttle_position: 0.0,
            tau,
            k,
        }
    }

    fn set_throttle_position(&mut self, position: f64) {
        self.throttle_position = position.clamp(0.0, 1.0);
    }

    fn update(&mut self, delta_time: f64) {
        let rpm_change = (self.k * self.throttle_position - self.current_rpm) / self.tau;
        self.current_rpm += rpm_change * delta_time;
    }

    fn get_rpm(&self) -> f64 {
        self.current_rpm
    }

    fn get_fuel_flow(&self) -> f64 {
        // RPM % N1, ex: 93.0 = MCT
        let numerator = (self.current_rpm / self.maximum_rpm) + 133.63;
        let denominator = 25.119; // JT8D
        let exponent = numerator / denominator;
        exponent.exp() // e^(exponent)
    }

    fn calculate_thrust(&self) -> f64 {
        let rpm_percentage_n1 = (self.current_rpm / self.maximum_rpm);
        let coefficient = 1258.7;
        let exponent_base = (0.0255 * rpm_percentage_n1).exp();
        coefficient * exponent_base * 1000.0
    }
}
