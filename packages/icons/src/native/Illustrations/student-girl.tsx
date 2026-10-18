import React from "react";
import type { Props } from "./shared";
import { StudentPortrait } from "./student-portrait";

export function StudentGirl({ size = 112 }: Props) {
  return <StudentPortrait size={size} girl />;
}
