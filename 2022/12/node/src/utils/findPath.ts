import { Node } from '../types'

const findPath = (node: Node, destination = 'E'): number => {
  let depth = 0
  let q: Set<Node> = new Set([node])

  while (q.size > 0) {
    const linksFromQ: Set<Node> = new Set([])
    for (const n of q) {
      n.visited = true
      if (n.name === destination) return depth

      n.links.forEach((l) => {
        if (!l.visited) {
          // if not using a set, we'd set l.visited to true here
          linksFromQ.add(l)
        }
      })
    }

    q = linksFromQ
    depth += 1
  }

  return -1
}

export default findPath
