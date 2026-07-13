export class PersonalInfoState {
  nameAr = $state("");
  nameEn = $state("");
  nationalId = $state("");
  sex = $state("male");
  classId = $state("");
  studentNumber = $state("");
  email = $state("");
  phone = $state("");
  address = $state("");
  status = $state("active");

  // Date picker components
  birthDay = $state("");
  birthMonth = $state("");
  birthYear = $state("");

  get birthDate() {
    if (this.birthDay && this.birthMonth && this.birthYear) {
      return `${this.birthYear}-${this.birthMonth.padStart(2, "0")}-${this.birthDay.padStart(2, "0")}`;
    }
    return "";
  }

  // Address and previous school
  motherName = $state("");
  motherPhone = $state("");
  motherJob = $state("");
  previousSchool = $state("");
  district = $state("");
  governorate = $state("");

  populate(originalStudent: any) {
    if (!originalStudent) return;
    this.nameAr = originalStudent.full_name || originalStudent.name_ar || "";
    this.nameEn = originalStudent.name_en || "";
    this.nationalId = originalStudent.national_id || "";

    const birth = originalStudent.birth_date
      ? String(originalStudent.birth_date).slice(0, 10)
      : "";
    if (birth) {
      const parts = birth.split("-");
      if (parts.length === 3) {
        this.birthYear = parts[0];
        this.birthMonth = String(parseInt(parts[1], 10));
        this.birthDay = String(parseInt(parts[2], 10));
      }
    }

    this.sex = originalStudent.sex || "male";
    this.classId = String(originalStudent.class_id || "");
    this.studentNumber = originalStudent.student_number || "";
    this.email = originalStudent.email || "";
    this.phone = originalStudent.phone_number || originalStudent.phone || "";
    this.address = originalStudent.address || "";
    this.status = originalStudent.status || "active";
  }
}
