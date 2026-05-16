export default function WaterSystem() {
  return (
    <mesh
      position={[0, 0, 0]}
      receiveShadow
    >
      <boxGeometry
        args={[200, 0.5, 200]}
      />
      <meshPhysicalMaterial
        color="#4ecbff"
        transparent
        opacity={0.7}
        roughness={0.2}
      />
    </mesh>
  );
}
