import type { Metadata } from "next";
import { ControlSettings } from "@/components/control-settings";

export const metadata: Metadata = {
  title: "Control Settings | Orbis",
};

export default function ControlPage() {
  return <ControlSettings />;
}
