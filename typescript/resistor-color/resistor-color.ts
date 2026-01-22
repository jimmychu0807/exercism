export const COLORS = [
  'black',
  'brown',
  'red',
  'orange',
  'yellow',
  'green',
  'blue',
  'violet',
  'grey',
  'white',
]

export const colorCode = (color:string): number => {
  // throw new Error('Remove this line and implement the function')
  return COLORS.indexOf(color.toLowerCase());
}


