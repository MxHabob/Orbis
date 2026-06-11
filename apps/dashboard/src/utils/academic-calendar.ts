export interface HijriMonth {
  id: number;
  name: string;
  label: string;
  isExam?: boolean;
  term?: number;
  isVacation?: boolean;
}

export const HIJRI_MONTHS: HijriMonth[] = [
  { id: 1, name: "محرم", label: "محرم (1)", term: 1 },
  { id: 2, name: "صفر", label: "صفر (2)", term: 1 },
  { id: 3, name: "ربيع أول", label: "ربيع أول (3)", term: 1 },
  { id: 4, name: "ربيع ثاني", label: "ربيع ثاني (4)", isExam: true, term: 1 },
  { id: 5, name: "جمادى أول", label: "جمادى أول (5)", term: 2 },
  { id: 6, name: "جمادى ثاني", label: "جمادى ثاني (6)", term: 2 },
  { id: 7, name: "رجب", label: "(7) رجب", term: 2 },
  { id: 8, name: "شعبان", label: "شعبان (8)", isExam: true, term: 2 },
  { id: 9, name: "رمضان", label: "رمضان (9)", isVacation: true },
  { id: 10, name: "شوال", label: "شوال (10)", isVacation: true },
  { id: 11, name: "ذو القعدة", label: "ذو القعدة (11)", isVacation: true },
  { id: 12, name: "ذو الحجة", label: "ذو الحجة (12)", isVacation: true },
];

export function getMonthName(id: number): string {
  return HIJRI_MONTHS.find((m) => m.id === id)?.name || `Month ${id}`;
}

export function getMonthLabel(id: number): string {
  return HIJRI_MONTHS.find((m) => m.id === id)?.label || `Month ${id}`;
}
