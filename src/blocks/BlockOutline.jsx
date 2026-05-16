export default function BlockOutline({ position }) {
  if (!position) return null;
  return (
    <mesh position={position}>
      <boxGeometry args={[1.02, 1.02, 1.02]} />
      <meshBasicMaterial
        color="white"
        wireframe
      />
    </mesh>
  );
}
