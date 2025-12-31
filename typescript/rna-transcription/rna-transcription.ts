export function toRna(dna: string): string {
  let rna = '';
  for (const realRna of dna) {
    switch (realRna) {
      case 'G': 
        rna += 'C'; 
        break;
      case 'C': 
        rna += 'G'; 
        break;
      case 'T': 
        rna += 'A'; 
        break;
      case 'A': 
        rna += 'U'; 
        break;
      default:
        throw new Error('Invalid input DNA.');
    }
  }
  return rna;
}