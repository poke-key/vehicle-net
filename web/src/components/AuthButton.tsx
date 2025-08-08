'use client';

import React from 'react';
import { useAuth } from '@/hooks/useAuth';
import { Button } from './ui/button';

export const AuthButton: React.FC = () => {
  const { user, isLoading, login, logout } = useAuth();

  if (isLoading) {
    return (
      <Button disabled>
        Connecting...
      </Button>
    );
  }

  if (user?.isAuthenticated) {
    return (
      <div className="flex items-center gap-4">
        <span className="text-sm text-gray-600">
          {user.address.slice(0, 6)}...{user.address.slice(-4)}
        </span>
        <Button variant="outline" onClick={logout}>
          Disconnect
        </Button>
      </div>
    );
  }

  return (
    <Button onClick={login}>
      Connect with Porto
    </Button>
  );
};