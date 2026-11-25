import type { StudentDetailDto } from "$lib/api";
import { FormController } from "$lib/shared/forms/form-controller.svelte";
import { studentRegistrationSchema } from "@orbis/schemas";

export class PersonalInfoState {
  private readonly registration = new FormController({
    initialValues: {
      name: "",
      name_en: "",
      national_id: "",
      gender: "",
      class_id: "",
      email: "",
      phone: "",
      address: "",
      nationality: "",
      passport_number: "",
    },
    schema: studentRegistrationSchema.pick({
      name: true,
      name_en: true,
      national_id: true,
      gender: true,
      class_id: true,
      email: true,
      phone: true,
      address: true,
      nationality: true,
      passport_number: true,
    }),
  });

  get nameAr() {
    return this.registration.values.name;
  }

  set nameAr(value: string) {
    this.registration.setField("name", value);
  }

  get nameEn() {
    return this.registration.values.name_en;
  }

  set nameEn(value: string) {
    this.registration.setField("name_en", value);
  }

  get nationalId() {
    return this.registration.values.national_id;
  }

  set nationalId(value: string) {
    this.registration.setField("national_id", value);
  }

  get sex() {
    return this.registration.values.gender;
  }

  set sex(value: string) {
    this.registration.setField("gender", value);
  }

  get classId() {
    return this.registration.values.class_id;
  }

  set classId(value: string) {
    this.registration.setField("class_id", value);
  }

  get email() {
    return this.registration.values.email;
  }

  set email(value: string) {
    this.registration.setField("email", value);
  }

  get phone() {
    return this.registration.values.phone;
  }

  set phone(value: string) {
    this.registration.setField("phone", value);
  }

  get address() {
    return this.registration.values.address;
  }

  set address(value: string) {
    this.registration.setField("address", value);
  }

  get nationality() {
    return this.registration.values.nationality;
  }

  set nationality(value: string) {
    this.registration.setField("nationality", value);
  }

  get passportNumber() {
    return this.registration.values.passport_number;
  }

  set passportNumber(value: string) {
    this.registration.setField("passport_number", value);
  }

  studentNumber = $state("");
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
  placeOfBirth = $state("");
  bloodType = $state("");
  emergencyContact = $state("");
  medicalNotes = $state("");
  specialNeedsNotes = $state("");

  populate(originalStudent: StudentDetailDto | null) {
    if (!originalStudent) return;
    const p = originalStudent.person;
    const s = originalStudent.student;
    const e = originalStudent.enrollment;

    this.nameAr =
      (p.first_name_ar && p.last_name_ar
        ? `${p.first_name_ar} ${p.last_name_ar}`
        : "") ||
      p.full_name_ar ||
      p.full_name ||
      "";
    this.nameEn =
      p.full_name ||
      (p.first_name && p.last_name ? `${p.first_name} ${p.last_name}` : "");
    this.nationalId = p.national_id || "";

    const birth = p.birth_date ? String(p.birth_date).slice(0, 10) : "";
    if (birth) {
      const parts = birth.split("-");
      if (parts.length === 3) {
        this.birthYear = parts[0];
        this.birthMonth = String(parseInt(parts[1], 10));
        this.birthDay = String(parseInt(parts[2], 10));
      }
    }

    this.sex = p.gender || "";
    this.classId = String(e?.section_id || "");
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
