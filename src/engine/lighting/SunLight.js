import * as THREE from "three";
export function createSunLight() {
  const light =
    new THREE.DirectionalLight(
      0xfff4d6,
      2
    );
  light.position.set(
    50,
    100,
    50
  );
  light.castShadow = true;
  return light;
}
