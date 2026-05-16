import * as THREE from "three";
export function createFog(scene) {
  scene.fog =
    new THREE.Fog(
      "#b8dfff",
      20,
      120
    );
}
