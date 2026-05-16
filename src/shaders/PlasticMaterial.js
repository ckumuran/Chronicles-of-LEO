import * as THREE from "three";

const materialCache={};

export function getPlasticMaterial(
color,
transparent=false
){

const key=
`${color}-${transparent}`;

if(materialCache[key]){

return materialCache[key];

}

const material=
new THREE.MeshPhysicalMaterial({

color,

roughness:0.35,

metalness:0.05,

clearcoat:0.4,

clearcoatRoughness:0.2,

transparent,

opacity:
transparent?0.7:1

});

materialCache[key]=
material;

return material;

}
