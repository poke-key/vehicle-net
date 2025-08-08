'use client';

import React from 'react';
import Link from 'next/link';
import { AuthButton } from './AuthButton';

export const Header: React.FC = () => {
  return (
    <header className="bg-white border-b border-gray-200 dark:bg-gray-900 dark:border-gray-800">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center py-4">
          <div className="flex items-center space-x-8">
            <Link href="/" className="flex items-center">
              <div className="w-8 h-8 bg-gray-900 dark:bg-white rounded-full flex items-center justify-center">
                <span className="text-white dark:text-gray-900 font-bold text-sm">V</span>
              </div>
              <span className="ml-2 text-lg font-semibold text-gray-900 dark:text-white">
                VehicleNet
              </span>
            </Link>
            <nav className="flex space-x-6">
              <Link href="/" className="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition-colors">
                Home
              </Link>
              <Link href="/marketplace" className="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition-colors">
                Marketplace
              </Link>
              <Link href="/list-vehicle" className="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition-colors">
                List Vehicle
              </Link>
              <Link href="/dashboard" className="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white transition-colors">
                Browse Data
              </Link>
            </nav>
          </div>
          <AuthButton />
        </div>
      </div>
    </header>
  );
};