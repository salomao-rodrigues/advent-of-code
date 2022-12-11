import { Directory, File } from './types'

const FILE_RE = new RegExp(/^\d+\s\S+$/)

export const createDirectory = (name: string): Directory => ({
  name,
  directories: [],
  files: []
})

export const addDirectoryIfNew = (
  currentDir: Directory,
  newDirName: string
): Directory => {
  let dir = currentDir.directories.find(({ name }) => name === newDirName)
  if (!dir) {
    dir = createDirectory(newDirName)
    currentDir.directories.push(dir)
    dir.parent = currentDir
  }

  return dir
}

export const addOrReplaceFile = (
  currentDir: Directory,
  name: string,
  size: number
): File => {
  const newFile: File = {
    name,
    size,
    parent: currentDir
  }
  const indexFound = currentDir.files.findIndex((f) => f.name === name)

  if (indexFound !== -1) {
    currentDir.files[indexFound] = newFile
  } else {
    currentDir.files.push(newFile)
  }

  return newFile
}

export const changeDirectory = (
  currentDir: Directory,
  name: string
): Directory => {
  if (name === '..') {
    if (!currentDir.parent) {
      throw new Error(
        `Current directory "${currentDir.name}" does not contain a parent directory.`
      )
    }
    return currentDir.parent
  }

  return addDirectoryIfNew(currentDir, name)
}

export const createTree = (commands: Array<string>): Directory => {
  const root: Directory = createDirectory('/')
  let current: Directory = root

  // First line is always "$ cd /" so we start from the second element
  commands.slice(1).forEach((command) => {
    if (command.indexOf('$ cd') === 0) {
      const [, , path] = command.split(' ')
      current = changeDirectory(current, path)
    } else if (command.indexOf('dir ') === 0) {
      const [, newDir] = command.split(' ')
      addDirectoryIfNew(current, newDir)
    } else if (FILE_RE.test(command)) {
      const [size, name] = command.split(' ')
      addOrReplaceFile(current, name, Number(size))
    }
  })

  return root
}

type TreeTotalSizes = {
  sum: number
  sumBelowThreshold: number
}
/*
 * Could do it all with one function but would have to return
 * an object or tuples
 */
export const recursiveTreeSum = (
  { directories, files }: Directory,
  threshold: number
): TreeTotalSizes => {
  let sum = 0
  let sumBelowThreshold = 0
  const [dirsSum, dirsSumBelowThreshold] = directories.reduce(
    (sum, d) => {
      const totals = recursiveTreeSum(d, threshold)
      return [sum[0] + totals.sum, sum[1] + totals.sumBelowThreshold]
    },
    [0, 0]
  )
  sumBelowThreshold += dirsSumBelowThreshold
  sum = dirsSum + files.reduce((sum, f) => sum + f.size, 0)

  if (sum <= threshold) {
    sumBelowThreshold += sum
  }
  return { sum, sumBelowThreshold }
}

export const findSizeOfSmallestEnoughDirectory = (
  node: Directory,
  threshold: number
): number => {
  let smallestEnoughSize = +Infinity

  ;(function traverseTree(node: Directory): number {
    let sum = node.directories.reduce((sum, d) => sum + traverseTree(d), 0)
    sum += node.files.reduce((sum, f) => sum + f.size, 0)

    if (sum >= threshold && sum < smallestEnoughSize) {
      smallestEnoughSize = sum
    }

    return sum
  })(node)

  return smallestEnoughSize
}
