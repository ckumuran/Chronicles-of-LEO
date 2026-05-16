import { getPlasticMaterial } from "../shaders/PlasticMaterial";

export default function BrickBlock({
position,
color,
onClick,
onContextMenu
}){

return(
<group
position={position}
onClick={onClick}
onContextMenu={onContextMenu}
>

<mesh
castShadow
receiveShadow
>

<boxGeometry
args={[1,1,1]}
/>

<primitive
object={
getPlasticMaterial(color)
}
attach="material"
/>

</mesh>

{[-0.25,0.25].map((x)=>
[-0.25,0.25].map((z)=>(

<mesh
key={`${x}-${z}`}
position={[x,0.55,z]}
castShadow
receiveShadow
>

<cylinderGeometry
args={[
0.12,
0.12,
0.08,
16
]}
/>

<primitive
object={
getPlasticMaterial(color)
}
attach="material"
/>

</mesh>

))
)}

</group>
);

}
