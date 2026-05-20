use noise::{
    NoiseFn,
    Perlin,
};

#[derive(
    Copy,
    Clone,
    PartialEq
)]
pub enum BiomeType {

    Plains,

    Desert,

    Mountains,

    Ocean,
}

pub struct BiomeGenerator {

    temperature: Perlin,

    moisture: Perlin,
}

impl BiomeGenerator {

    pub fn new() -> Self {

        Self {

            temperature:
                Perlin::new(1234),

            moisture:
                Perlin::new(5678),
        }
    }

    pub fn get_biome(
        &self,

        x: i32,
        z: i32,
    ) -> BiomeType {

        let scale = 0.002;

        let temp =
            self.temperature.get([
                x as f64 * scale,
                z as f64 * scale,
            ]);

        let moisture =
            self.moisture.get([
                x as f64 * scale,
                z as f64 * scale,
            ]);

        // OCEAN

        if moisture > 0.5 &&
           temp < -0.2
        {
            return BiomeType::Ocean;
        }

        // DESERT

        if temp > 0.4 &&
           moisture < 0.0
        {
            return BiomeType::Desert;
        }

        // MOUNTAINS

        if temp < -0.3
        {
            return BiomeType::Mountains;
        }

        BiomeType::Plains
    }
}
