import { PointerLockControls } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import { useRef } from "react";
import * as THREE from "three";
import movement from "./Controls";
export default function Player() {
const controlsRef=useRef();
const velocity=useRef(new THREE.Vector3());
useFrame((state,delta)=>{
if(!controlsRef.current)return;
const speed=movement.sprint?10:5;
const direction=new THREE.Vector3();
if(movement.forward)direction.z-=1;
if(movement.backward)direction.z+=1;
if(movement.left)direction.x-=1;
if(movement.right)direction.x+=1;
direction.normalize();
velocity.current.x=direction.x*speed*delta;
velocity.current.z=direction.z*speed*delta;
controlsRef.current.moveRight(
velocity.current.x
);
controlsRef.current.moveForward(
velocity.current.z
);
});
return(
<PointerLockControls
ref={controlsRef}
/>
);
}
