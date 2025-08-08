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
    <header className="sticky top-0 z-50 w-full border-b border-border/40 bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="flex h-14 items-center justify-between px-6 max-w-[1200px] mx-auto">
        {/* Logo on the left */}
        <div className="flex items-center">
          <Link className="flex items-center space-x-3" href="/">
            <div className="flex h-7 w-7 items-center justify-center rounded-full bg-foreground">
              <Car className="h-3.5 w-3.5 text-background" />
            </div>
            <span className="font-medium text-foreground">VehicleNet</span>
          </Link>
        </div>
        
        {/* Navigation and controls on the right */}
        <div className="flex items-center space-x-8">
          <nav className="flex items-center space-x-8 text-sm">
            {links.map(({ to, label }) => (
              <Link
                key={to}
                href={to}
                className={cn(
                  "transition-colors hover:text-foreground/80",
                  pathname === to 
                    ? "rounded-md bg-muted/80 px-3 py-1.5 text-foreground font-medium" 
                    : "text-muted-foreground"
                )}
              >
                {label}
              </Link>
            ))}
          </nav>
          
          <div className="flex items-center space-x-6">
            <Button variant="outline" size="sm" className="h-8 px-3" asChild>
              <Link href="/dashboard">
                Browse Data
              </Link>
            </Button>
            <ConnectWallet />
            <ModeToggle />
          </div>
        </div>
      </div>
    </header>
  );
}
