export type Node = {
  name: string
  links: Array<Node>
  visited?: boolean
}

export type HeightMap = Array<Array<Node>>
