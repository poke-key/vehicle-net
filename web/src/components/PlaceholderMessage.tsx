'use client';

import React from 'react';
import { Card } from './ui/card';
import { AuthButton } from './AuthButton';

export const PlaceholderMessage: React.FC = () => {
  return (
    <div className="flex items-center justify-center min-h-[400px]">
      <Card className="text-center max-w-md">
        <div className="mb-6">
          <div className="mx-auto w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-4">
            <svg 
              className="w-8 h-8 text-gray-400" 
              fill="none" 
              stroke="currentColor" 
              viewBox="0 0 24 24"
            >
              <path 
                strokeLinecap="round" 
                strokeLinejoin="round" 
                strokeWidth={2} 
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" 
              />
            </svg>
          </div>
          <h3 className="text-lg font-semibold text-gray-900 mb-2">
            Authentication Required
          </h3>
          <p className="text-gray-600 mb-6">
            Please authenticate with Porto to access vehicle data listings and marketplace features.
          </p>
        </div>
        <AuthButton />
      </Card>
    </div>
  );
};