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
  previousSchool = $state("");
  district = $state("");
  governorate = $state("");

  // Additional fields
  nationality = $state("");
  placeOfBirth = $state("");
  passportNumber = $state("");
  bloodType = $state("");
  emergencyContact = $state("");
  medicalNotes = $state("");

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
    this.previousSchool =
      originalStudent.previous_school || originalStudent.previousSchool || "";
    this.district = originalStudent.district || "";
    this.governorate = originalStudent.governorate || "";
    this.nationality = originalStudent.nationality || "";
    this.placeOfBirth = originalStudent.place_of_birth || "";
    this.passportNumber = originalStudent.passport_number || "";
    this.bloodType = originalStudent.blood_type || "";
    this.emergencyContact = originalStudent.emergency_contact || "";
    this.medicalNotes = originalStudent.medical_notes || "";
    this.status = originalStudent.status || "active";
  }
}
