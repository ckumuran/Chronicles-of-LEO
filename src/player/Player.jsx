import { PointerLockControls } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import { useRef } from "react";
import * as THREE from "three";
import movement from "./Controls";
import PhysicsEngine from "../engine/physics/PhysicsEngine";
export default function Player(){
const controlsRef=useRef();
const physics=useRef(
new PhysicsEngine()
);
const velocity=useRef(
new THREE.Vector3(0,0,0)
);
useFrame((state,delta)=>{
if(!controlsRef.current)return;
const player=
controlsRef.current.object;
const speed=
movement.sprint?12:6;
const direction=
new THREE.Vector3();
if(movement.forward)
direction.z-=1;
if(movement.backward)
direction.z+=1;
if(movement.left)
direction.x-=1;
if(movement.right)
direction.x+=1;
direction.normalize();
velocity.current.x=
direction.x*speed;
velocity.current.z=
direction.z*speed;
if(
movement.jump &&
player.position.y<=2.01
){
velocity.current.y=8;
}
player.position.x+=
velocity.current.x*delta;
player.position.z+=
velocity.current.z*delta;
physics.current.update(
player,
velocity.current,
delta
);
});
return(
<PointerLockControls
ref={controlsRef}
/>
);
}
