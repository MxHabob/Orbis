import { ControlSettings } from "@/components/control-settings";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Control Settings | Orbis",
};

export default function ControlPage() {
  return <ControlSettings />;
}
