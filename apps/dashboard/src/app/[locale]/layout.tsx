import "@/styles/globals.css";
import { cn } from "@orbis/ui/cn";
import "@orbis/ui/globals.css";
import { DesktopHeader } from "@/components/desktop-header";
import { isDesktopApp } from "@/utils/desktop";
import { Toaster } from "@orbis/ui/toaster";
import type { Metadata } from "next";
import { Hedvig_Letters_Sans, Hedvig_Letters_Serif } from "next/font/google";
import { NuqsAdapter } from "nuqs/adapters/next/app";
import type { ReactElement } from "react";
import { Providers } from "./providers";

export const metadata: Metadata = {
  metadataBase: new URL("https://app.orbis.com"),
  title: "Orbis | School Management",
  description:
    "Automate School tasks, stay organized, and make informed decisions effortlessly.",
  twitter: {
    title: "Orbis | School Management",
    description:
      "Automate School tasks, stay organized, and make informed decisions effortlessly.",
    images: [
      {
        url: "https://cdn.orbis.ai/opengraph-image-v1.jpg",
        width: 800,
        height: 600,
      },
      {
        url: "https://cdn.orbis.ai/opengraph-image-v1.jpg",
        width: 1800,
        height: 1600,
      },
    ],
  },
  openGraph: {
    title: "Orbis | School Management",
    description:
      "Automate financial tasks, stay organized, and make informed decisions effortlessly.",
    url: "https://app.orbis.com",
    siteName: "Orbis",
    images: [
      {
        url: "https://cdn.orbis.ai/opengraph-image-v1.jpg",
        width: 800,
        height: 600,
      },
      {
        url: "https://cdn.orbis.ai/opengraph-image-v1.jpg",
        width: 1800,
        height: 1600,
      },
    ],
    locale: "en_US",
    type: "website",
  },
};

const hedvigSans = Hedvig_Letters_Sans({
  weight: "400",
  subsets: ["latin"],
  display: "swap",
  variable: "--font-hedvig-sans",
});

const hedvigSerif = Hedvig_Letters_Serif({
  weight: "400",
  subsets: ["latin"],
  display: "swap",
  variable: "--font-hedvig-serif",
});

export const viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 1,
  userScalable: false,
  themeColor: [
    { media: "(prefers-color-scheme: light)" },
    { media: "(prefers-color-scheme: dark)" },
  ],
};

export default async function Layout({
  children,
  params,
}: {
  children: ReactElement;
  params: Promise<{ locale: string }>;
}) {
  const { locale } = await params;
  const isDesktop = await isDesktopApp();

  return (
    <html
      lang={locale}
      dir={locale === "ar" ? "rtl" : "ltr"}
      suppressHydrationWarning
      className={cn(isDesktop && "desktop")}
    >
      <body
        className={cn(
          `${hedvigSans.variable} ${hedvigSerif.variable} font-sans`,
          "whitespace-pre-line overscroll-none antialiased",
        )}
      >
        <DesktopHeader />
        <NuqsAdapter>
          <Providers locale={locale}>
            {children}
            <Toaster />
          </Providers>
        </NuqsAdapter>
      </body>
    </html>
  );
}
