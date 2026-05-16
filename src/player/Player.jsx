import { PointerLockControls } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import { useRef } from "react";
import * as THREE from "three";
export default function Player() {
  const controlsRef = useRef();
  const velocity = useRef(
    new THREE.Vector3()
  );
  const movement = useRef({
    forward: false,
    backward: false,
    left: false,
    right: false,
    sprint: false
  });
  window.onkeydown = (e) => {
    switch (e.code) {
      case "KeyW":
        movement.current.forward = true;
        break;
      case "KeyS":
        movement.current.backward = true;
        break;
      case "KeyA":
        movement.current.left = true;
        break;
      case "KeyD":
        movement.current.right = true;
        break;
      case "ShiftLeft":
        movement.current.sprint = true;
        break;
    }
  };
  window.onkeyup = (e) => {
    switch (e.code) {
      case "KeyW":
        movement.current.forward = false;
        break;
      case "KeyS":
        movement.current.backward = false;
        break;
      case "KeyA":
        movement.current.left = false;
        break;
      case "KeyD":
        movement.current.right = false;
        break;
      case "ShiftLeft":
        movement.current.sprint = false;
        break;
    }
  };
  useFrame((state, delta) => {
    if (!controlsRef.current) return;
    const speed =
      movement.current.sprint
        ? 10
        : 5;
    const direction =
      new THREE.Vector3();
    if (movement.current.forward)
      direction.z -= 1;
    if (movement.current.backward)
      direction.z += 1;
    if (movement.current.left)
      direction.x -= 1;
    if (movement.current.right)
      direction.x += 1;
    direction.normalize();
    velocity.current.x =
      direction.x * speed * delta;
    velocity.current.z =
      direction.z * speed * delta;
    controlsRef.current.moveRight(
      velocity.current.x
    );
    controlsRef.current.moveForward(
      velocity.current.z
    );
  });
  return (
    <PointerLockControls
      ref={controlsRef}
    />
  );
}
