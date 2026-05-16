import { BlockRegistry }
from "../world/BlockRegistry";

export const colors={};

for(const key in BlockRegistry){

colors[key]=
BlockRegistry[key].color;

}
