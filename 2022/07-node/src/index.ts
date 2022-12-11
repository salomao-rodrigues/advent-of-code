import { readFileSync } from 'fs'
import {
  createTree,
  findSizeOfSmallestEnoughDirectory,
  recursiveTreeSum
} from './tree'

const [filePath] = process.argv.slice(2)
const lines = readFileSync(filePath, 'utf-8').split(/\r?\n/)
const tree = createTree(lines)

const TOTAL_DISK_SPACE = 70000000
const SPACE_NEEDED = 30000000
const { sum: usedSpace, sumBelowThreshold } = recursiveTreeSum(tree, 100000)
const minimumSizeNeededToFree = SPACE_NEEDED - (TOTAL_DISK_SPACE - usedSpace)
const fileSizeToDelete = findSizeOfSmallestEnoughDirectory(
  tree,
  minimumSizeNeededToFree
)

console.log('Part 1:', sumBelowThreshold)
console.log('Part 2:', fileSizeToDelete)
