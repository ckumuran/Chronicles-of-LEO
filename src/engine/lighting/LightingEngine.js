import * as THREE from "three";

export function setupLighting(
scene
){

const ambient=
new THREE.AmbientLight(
0xffffff,
0.7
);

scene.add(ambient);

const sun=
new THREE.DirectionalLight(
0xfff2cc,
2
);

sun.position.set(
50,
80,
30
);

sun.castShadow=true;

sun.shadow.mapSize.width=
2048;

sun.shadow.mapSize.height=
2048;

sun.shadow.camera.near=1;

sun.shadow.camera.far=200;

sun.shadow.camera.left=-100;

sun.shadow.camera.right=100;

sun.shadow.camera.top=100;

sun.shadow.camera.bottom=-100;

scene.add(sun);

const hemisphere=
new THREE.HemisphereLight(
0xb1e1ff,
0x444422,
0.5
);

scene.add(hemisphere);

scene.fog=
new THREE.Fog(
"#b8dfff",
40,
180
);

}
