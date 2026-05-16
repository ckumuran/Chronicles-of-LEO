import * as THREE from "three";
export function setupLighting(scene) {
  const ambient =
    new THREE.AmbientLight(
      0xffffff,
      0.8
    );
  scene.add(ambient);
  const directional =
    new THREE.DirectionalLight(
      0xfff2cc,
      1.5
    );
  directional.position.set(
    20,
    30,
    20
  );
  directional.castShadow = true;
  scene.add(directional);
}
