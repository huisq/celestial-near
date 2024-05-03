"use client"
import './globals.css'
import { Inter } from 'next/font/google'
import { useState } from "react";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {

  const [showImport, setShowImport] = useState<boolean>(false);

  return (
    <html lang="en">
      <body >
          {children}
      </body>
    </html>
  )
}