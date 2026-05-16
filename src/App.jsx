import { Canvas,useThree } from "@react-three/fiber";

import { useEffect } from "react";

import World from "./world/World";

import Player from "./player/Player";

import Crosshair from "./ui/Crosshair";

import { setupLighting }
from "./engine/lighting/LightingEngine";

function SceneSetup(){

const { scene }=useThree();

useEffect(()=>{

setupLighting(scene);

},[scene]);

return null;

}

export default function App(){

return(
<>

<Canvas
shadows
camera={{
fov:75,
position:[0,5,10]
}}
>

<SceneSetup />

<World />

<Player />

</Canvas>

<Crosshair />

</>
);

}
