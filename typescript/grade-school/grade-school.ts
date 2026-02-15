export class GradeSchool {
  
  private studentsList: { [grade: number]: string[] } = {}; //定义名册，以年级对应数组的形式记录

  roster(): {[grade: number]: string[]} { 
    const gradesList: number[] = [];
    for (const gradeName in this.studentsList) {
      const gradeNum = parseInt(gradeName);
      gradesList.push(gradeNum);
    }

    gradesList.sort((a, b) => a - b); // 排序

    const result: { [grade: number]: string[] } = {};

    for (let i = 0; i < gradesList.length; i++) { // 按年级顺序整理名单
      const currentGrade = gradesList[i]; 
      const newList = [];       
      for (let j = 0; j < this.studentsList[currentGrade].length; j++) {
        newList.push(this.studentsList[currentGrade][j]);  
      }
      result[currentGrade] = newList; 
    }
    
    return result;    

  }   

  add(student: string, grade: number): void {

    for (const g in this.studentsList) { // 遍历年级
      const currentGrade = parseInt(g);
      if (this.studentsList[currentGrade].includes(student)) {
        // 先检查当前输入的学生名字有没有在遍历到的当前年级出现
        const index = this.studentsList[currentGrade].indexOf(student); // 找到出现的位置
        this.studentsList[currentGrade].splice(index, 1); // 有的话删掉
        break;
      }
    }

    if (!this.studentsList[grade]) {
      this.studentsList[grade] = []; // 创建年级
    }
    this.studentsList[grade].push(student); // 在年级里加入学生
    this.studentsList[grade].sort(); // 排序
  }  

  grade(grade: number): string[] {
    if (this.studentsList[grade]) { // 如果查询到指定年级
      const newList = []; 
      for (let i = 0; i < this.studentsList[grade].length; i++) {
        newList.push(this.studentsList[grade][i]);
      }
      return newList; // 方法雷同 .roster(), 但是根据指定年级生成名册
    } else {
      return [];
    }
  }
}
