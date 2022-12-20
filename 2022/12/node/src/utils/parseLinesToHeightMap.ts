import { Node } from '../types'

const parseLinesToHeightMap = (lines: Array<string>): Array<Array<Node>> => {
  return lines.map((l) => l.split('').map((name) => ({ name, links: [] })))
}

export default parseLinesToHeightMap
