import { readFileSync } from 'fs'
import createLinks, { ORIENTATION } from './utils/createLinks'
import findPath from './utils/findPath'
import getStartNode from './utils/getStartNode'
import parseLinesToHeightMap from './utils/parseLinesToHeightMap'

// Plan to solve this thing...
// We're building a Linked List data structure.
// [x] Parse file to a matrix of Nodes
// [x] Create links between Nodes. Only link if height diff <= max diff allowed
// [x] Find the starting node
// [x] Use a BFS (Breadth-First Search) algorithm to find the shortest path.

const [filePath] = process.argv.slice(2)
const lines = readFileSync(filePath, 'utf-8').trim().split(/\r?\n/)

// Create map matrix
const map1 = parseLinesToHeightMap(lines)
const map2 = parseLinesToHeightMap(lines)

// Part 1
createLinks(map1, ORIENTATION.ASC)
const startNode = getStartNode(map1)
console.log('Part 1:', findPath(startNode))

// Part 2
createLinks(map2, 1, ORIENTATION.DESC)
const endNode = getStartNode(map2, 'E')
console.log('Part 2:', findPath(endNode, 'a'))
