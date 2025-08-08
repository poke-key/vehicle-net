'use client';

import React from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Car } from 'lucide-react';
import { AuthButton } from './AuthButton';
import { ModeToggle } from './mode-toggle';

export const Header: React.FC = () => {
  const pathname = usePathname();

  const navItems = [
    { href: '/', label: 'Home' },
    { href: '/vehicles', label: 'Vehicles' },
    { href: '/dashboard', label: 'Dashboard' },
  ];

  return (
    <header className="bg-background border-b border-border">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center py-4">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-2">
            <div className="w-8 h-8 bg-white rounded-full flex items-center justify-center border border-gray-300">
              <Car className="h-5 w-5 text-black" />
            </div>
            <span className="text-xl font-bold text-foreground">VehicleNet</span>
          </Link>

          {/* Navigation Links */}
          <nav className="hidden md:flex items-center space-x-6">
            {navItems.map((item) => {
              const isActive = pathname === item.href || 
                (item.href !== '/' && pathname.startsWith(item.href));
              
              return (
                <Link
                  key={item.href}
                  href={item.href}
                  className={`px-3 py-2 rounded-md text-sm font-medium transition-colors ${
                    isActive
                      ? 'bg-muted text-foreground'
                      : 'text-muted-foreground hover:text-foreground hover:bg-muted/50'
                  }`}
                >
                  {item.label}
                </Link>
              );
            })}
          </nav>

          {/* Right side - Connect Wallet and Theme Toggle */}
          <div className="flex items-center space-x-4">
            <AuthButton />
            <ModeToggle />
          </div>
        </div>
      </div>
    </header>
  );
};
