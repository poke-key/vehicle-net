'use client';

import React from 'react';
import { AuthButton } from './AuthButton';

const Header: React.FC = () => {
  return (
    <header className="bg-white border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center py-4">
          <div className="flex items-center">
            <h1 className="text-xl font-bold text-gray-900">
              Vehicle Data Marketplace
            </h1>
          </div>
          <AuthButton />
        </div>
      </div>
    </header>
  );
};

export default Header;