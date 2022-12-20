import { Node } from '../types'

const createNode = (name: string): Node => {
  if (!name.match(/^[a-zSE]$/)) {
    throw Error(
      `Node ${name} is not a valid height. Valid types are lower case letters (a-z) and S (start) or E (end)`
    )
  }
  return {
    name,
    links: []
  }
}

export default createNode
