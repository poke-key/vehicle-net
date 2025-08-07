"use client";

import Link from "next/link";
import { Car, Github, Twitter } from "lucide-react";

export default function Footer() {
  return (
    <footer className="border-t bg-muted/40">
      <div className="container flex flex-col items-center justify-between gap-4 py-10 md:h-24 md:flex-row md:py-0">
        <div className="flex flex-col items-center gap-4 px-8 md:flex-row md:gap-2 md:px-0">
          <Car className="h-6 w-6" />
          <p className="text-center text-sm leading-loose text-muted-foreground md:text-left">
            Built on Ethereum. Powered by decentralization. VehicleNet © 2024.
          </p>
        </div>
        <div className="flex items-center space-x-4">
          <Link
            href="https://github.com"
            target="_blank"
            rel="noreferrer"
            className="text-muted-foreground hover:text-foreground"
          >
            <Github className="h-5 w-5" />
          </Link>
          <Link
            href="https://twitter.com"
            target="_blank"
            rel="noreferrer"
            className="text-muted-foreground hover:text-foreground"
          >
            <Twitter className="h-5 w-5" />
          </Link>
        </div>
      </div>
    </footer>
  );
}