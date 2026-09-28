import type { StudentDetailDto } from "$lib/api";

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
  specialNeedsNotes = $state("");

  populate(originalStudent: StudentDetailDto | null) {
    if (!originalStudent) return;
    const p = originalStudent.person;
    const s = originalStudent.student;
    const e = originalStudent.enrollment;

    this.nameAr = (p.first_name_ar && p.last_name_ar ? `${p.first_name_ar} ${p.last_name_ar}` : "") || p.full_name_ar || p.full_name || "";
    this.nameEn = p.full_name || (p.first_name && p.last_name ? `${p.first_name} ${p.last_name}` : "");
    this.nationalId = p.national_id || "";

    const birth = p.birth_date
      ? String(p.birth_date).slice(0, 10)
      : "";
    if (birth) {
      const parts = birth.split("-");
      if (parts.length === 3) {
        this.birthYear = parts[0];
        this.birthMonth = String(parseInt(parts[1], 10));
        this.birthDay = String(parseInt(parts[2], 10));
      }
    }

    this.sex = p.gender || "male";
    this.classId = String(e?.class_id || "");
    this.studentNumber = s.student_number || "";
    this.email = p.email || "";
    this.phone = p.phone || "";
    this.address = p.address || "";
    this.nationality = p.nationality || "";
    this.placeOfBirth = p.place_of_birth || "";
    this.passportNumber = p.passport_number || "";
    this.bloodType = p.blood_type || "";
    this.medicalNotes = p.medical_notes || "";
    this.specialNeedsNotes = p.special_needs_notes || "";
    this.status = s.status || "active";
  }
}
