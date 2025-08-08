"use client"

import { VehicleListings } from '@/components/VehicleListings';
import { useAuth } from '@/hooks/useAuth';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Shield, AlertCircle } from 'lucide-react';

export default function MarketplacePage() {
  const { user, isLoading, error, login } = useAuth();

  if (isLoading) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex items-center justify-center min-h-[400px]">
          <div className="w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin" />
          <span className="ml-2 text-gray-600">Checking authentication...</span>
        </div>
      </div>
    );
  }

  if (!user?.isAuthenticated) {
    return (
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="max-w-md mx-auto">
          <Card>
            <CardHeader className="text-center">
              <Shield className="h-12 w-12 text-yellow-500 mx-auto mb-4" />
              <CardTitle>Authentication Required</CardTitle>
              <CardDescription>
                You need to sign in with Porto to view vehicle data from the blockchain.
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {error && (
                <div className="bg-red-50 border border-red-200 rounded-md p-4">
                  <div className="flex items-center">
                    <AlertCircle className="h-5 w-5 text-red-400 mr-2" />
                    <span className="text-sm text-red-600">{error}</span>
                  </div>
                </div>
              )}
              <Button 
                onClick={login} 
                className="w-full"
                size="lg"
              >
                Sign in with Porto
              </Button>
              <p className="text-xs text-center text-gray-500">
                Porto provides secure blockchain authentication and payment processing.
              </p>
            </CardContent>
          </Card>
        </div>
      </div>
    );
  }

  return <VehicleListings />;
}