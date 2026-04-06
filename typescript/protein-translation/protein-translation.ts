const aminoAcidToCodon: { [key: string]: string[] } = {
  Methionine: ["AUG"],
  Phenylalanine: ["UUU", "UUC"],
  Leucine: ["UUA", "UUG"],
  Serine: ["UCU", "UCC", "UCA", "UCG"],
  Tyrosine: ["UAU", "UAC"],
  Cysteine: ["UGU", "UGC"],
  Tryptophan: ["UGG"],
  STOP: ["UAA", "UAG", "UGA"],
};

export function translate(rna: string): string[] {
  const result: string[] = [];

  for (let i = 0; i < rna.length; i += 3) {
    const codon = rna.slice(i, i + 3); // 把输入项分割为每三个为一位去对比
    let protein: string | undefined = undefined; // 如果后边找到对应的 Amino Acid ，就用 Amino Acid 给它赋值，如果找不到，就赋值为 undefined 。需要先假设它是 underfined。不然就没法应对后边 Invalid codon 的情况
    for (const aminoAcid in aminoAcidToCodon) {
      if (aminoAcidToCodon[aminoAcid].includes(codon)) {
        // 找 aminoAcidToCodon 里，包含输入值所在的那个 value 对应的 key
        protein = aminoAcid;
        break;
      }
    }

    if (protein === undefined) {
      throw new Error("Invalid codon");
    }

    if (protein === "STOP") {
      return result;
      // 本来用 break 也没有报错，但是原文写了 "Once we reach that point, we stop processing." 所以到这里应该直接返回当前结果
    }

    result.push(protein);
  }

  return result;
}
