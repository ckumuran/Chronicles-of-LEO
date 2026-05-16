import SimplexNoise from "simplex-noise";
const noise = new SimplexNoise();
export function getHeight(x, z) {
  return Math.floor(
    (noise.noise2D(x / 20, z / 20) + 1) * 4 + 2
  );
}
