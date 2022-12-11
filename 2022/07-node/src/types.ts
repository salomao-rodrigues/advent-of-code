export enum FileTypes {
  DIRECTORY,
  FILE
}

export type File = {
  name: string
  size: number
  parent: Directory
}

export type Directory = {
  name: string
  directories: Array<Directory>
  files: Array<File>
  parent?: Directory
}
