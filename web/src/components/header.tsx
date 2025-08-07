"use client";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { Car } from "lucide-react";
import { Button } from "./ui/button";
import { ModeToggle } from "./mode-toggle";
import { ConnectWallet } from "./connect-wallet";
import { cn } from "@/lib/utils";

export default function Header() {
  const pathname = usePathname();
  
  const links = [
    { to: "/", label: "Home" },
    { to: "/dashboard", label: "Marketplace" },
    { to: "/list-vehicle", label: "List Vehicle" },
  ];

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-14 items-center">
        <div className="mr-4 flex">
          <Link className="mr-6 flex items-center space-x-2" href="/">
            <Car className="h-6 w-6" />
            <span className="hidden font-bold sm:inline-block">VehicleNet</span>
          </Link>
          <nav className="flex items-center space-x-6 text-sm font-medium">
            {links.map(({ to, label }) => (
              <Link
                key={to}
                href={to}
                className={cn(
                  "transition-colors hover:text-foreground/80",
                  pathname === to ? "text-foreground" : "text-foreground/60"
                )}
              >
                {label}
              </Link>
            ))}
          </nav>
        </div>
        <div className="flex flex-1 items-center justify-between space-x-2 md:justify-end">
          <div className="w-full flex-1 md:w-auto md:flex-none">
            <Button variant="outline" className="hidden md:inline-flex" size="sm" asChild>
              <Link href="/dashboard">
                Browse Data
              </Link>
            </Button>
          </div>
          <nav className="flex items-center space-x-2">
            <ConnectWallet />
            <ModeToggle />
          </nav>
        </div>
      </div>
    </header>
  );
}
