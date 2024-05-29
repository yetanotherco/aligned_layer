import { SimpleMerkleTree } from '@openzeppelin/merkle-tree';
import { keccak256 } from '@ethersproject/keccak256';

const child_1 = keccak256(1);
const child_2 = keccak256(2);

console.log("KECCAK256 OF 1:", child_1)
console.log("KECCAK256 OF 2:", child_2)

const tree = SimpleMerkleTree.of([child_1, child_2]);

console.log('MERKLE ROOT:', tree.root);
