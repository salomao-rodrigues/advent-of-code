import { TEST_CASE_1 } from './mocks/commands'
import {
  addDirectoryIfNew,
  addOrReplaceFile,
  changeDirectory,
  createDirectory,
  createTree,
  findSizeOfSmallestEnoughDirectory,
  recursiveTreeSum
} from './tree'
import { Directory } from './types'

describe('tree', () => {
  let root: Directory
  beforeEach(() => (root = createDirectory('/')))

  describe('createDirectory', () => {
    it('creates the root folder', () => {
      const expected: Directory = {
        name: '/',
        directories: [],
        files: []
      }
      expect(root).toEqual(expected)
    })
  })

  describe('addDirectoryIfNew', () => {
    it("adds sub directory to parent's directories list", () => {
      const sub = addDirectoryIfNew(root, 'sub')

      expect(root.directories).toContain(sub)
    })

    it('does NOT override the existing sub dir if it already exists', () => {
      const sub = addDirectoryIfNew(root, 'sub')
      sub.directories.push(createDirectory('nested'))

      const existing = addDirectoryIfNew(root, 'sub')

      expect(existing).toBe(sub)
    })
  })

  describe('changeDirectory', () => {
    describe('when given a sub directory', () => {
      it('adds the correct directory fields', () => {
        const current = changeDirectory(root, 'sub')
        const expected: Directory = {
          name: 'sub',
          directories: [],
          files: [],
          parent: root
        }
        expect(current).toEqual(expected)
      })

      it("throws an exception when trying to navigate to parent directory but one doesn't exist", () => {
        expect(() => changeDirectory(root, '..')).toThrow()
      })

      it('navigates to parent directory', () => {
        // first we navigate to a sub folder
        const sub = changeDirectory(root, 'sub')
        expect(sub).toEqual({
          name: 'sub',
          directories: [],
          files: [],
          parent: root
        })

        // then we navigate back up to the root
        expect(changeDirectory(sub, '..')).toEqual(root)
      })
    })
  })

  describe('addOrReplaceFile', () => {
    it('adds multiple files', () => {
      const file1 = addOrReplaceFile(root, 'a', 123)
      expect(root).toEqual({
        name: '/',
        directories: [],
        files: [file1]
      })

      const file2 = addOrReplaceFile(root, 'b', 321)
      expect(root).toEqual({
        name: '/',
        directories: [],
        files: [file1, file2]
      })
    })

    it('adds a file and overrides it if it exists', () => {
      addOrReplaceFile(root, 'a', 123)
      const file2 = addOrReplaceFile(root, 'b', 321)
      addOrReplaceFile(root, 'a', 43)

      expect(root).toEqual({
        name: '/',
        directories: [],
        files: [
          {
            name: 'a',
            size: 43,
            parent: root
          },
          file2
        ]
      })
    })
  })

  describe('recursiveTreeSum', () => {
    it('sums folder sizes that are within a given threshold (included)', () => {
      const tree = createTree(TEST_CASE_1.split('\n'))
      expect(recursiveTreeSum(tree, 100000).sumBelowThreshold).toBe(95437)
    })
  })

  describe('findSizeOfSmallestEnoughDirectory', () => {
    it('find smallets folder that is above threshold', () => {
      const tree = createTree(TEST_CASE_1.split('\n'))
      expect(findSizeOfSmallestEnoughDirectory(tree, 8381165)).toBe(24933642)
    })
  })
})
