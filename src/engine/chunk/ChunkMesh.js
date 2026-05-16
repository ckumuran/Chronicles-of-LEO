import * as THREE from "three";
export function createChunkMesh(blocks) {
  const geometry = new THREE.BoxGeometry(1, 1, 1);
  const material = new THREE.MeshStandardMaterial({
    color: "white"
  });
  const meshes = [];
  for (const block of blocks) {
    const mesh = new THREE.Mesh(
      geometry,
      material
    );
    mesh.position.set(
      block.x,
      block.y,
      block.z
    );
    meshes.push(mesh);
  }
  return meshes;
}
