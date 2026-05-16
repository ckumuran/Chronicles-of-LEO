import * as THREE from "three";

export function createChunkMesh(
blocks,
colors
){

const group=
new THREE.Group();

for(const block of blocks){

const geometry=
new THREE.BoxGeometry(
1,
1,
1
);

const material=
new THREE.MeshStandardMaterial({

color:
colors[block.type],

roughness:0.35

});

const mesh=
new THREE.Mesh(
geometry,
material
);

mesh.position.set(
block.x,
block.y,
block.z
);

mesh.castShadow=true;
mesh.receiveShadow=true;

group.add(mesh);

}

return group;

}
