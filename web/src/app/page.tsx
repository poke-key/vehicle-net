"use client"

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Car, Shield, Zap, DollarSign, Users, TrendingUp } from "lucide-react";
import Link from "next/link";
import { ConnectWallet } from "@/components/connect-wallet";

export default function Home() {
  // Always show landing page - marketplace has its own dedicated page
  return (
    <div className="min-h-screen bg-gradient-to-b from-background to-muted/20">
      {/* Hero Section */}
      <section className="container mx-auto px-4 py-16 text-center">
        <div className="mx-auto max-w-4xl">
          <h1 className="mb-6 text-4xl font-bold tracking-tight md:text-6xl">
            Decentralized Vehicle Data 
            <span className="text-primary"> Marketplace</span>
          </h1>
          <p className="mb-8 text-lg text-muted-foreground md:text-xl">
            Buy and sell real-time vehicle data streams securely on the blockchain. 
            Connect directly with vehicle owners and data consumers in a trustless environment.
          </p>
          <div className="flex flex-col gap-4 sm:flex-row sm:justify-center">
            <ConnectWallet />
            <Link href="/marketplace">
              <Button variant="outline" size="lg">
                Explore Marketplace
              </Button>
            </Link>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="container mx-auto px-4 py-16">
        <div className="mx-auto max-w-6xl">
          <h2 className="mb-12 text-center text-3xl font-bold">Why Choose VehicleNet?</h2>
          <div className="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
            <Card>
              <CardHeader>
                <Shield className="mb-2 h-10 w-10 text-primary" />
                <CardTitle>Secure & Decentralized</CardTitle>
                <CardDescription>
                  Smart contracts ensure trustless transactions and data integrity
                </CardDescription>
              </CardHeader>
            </Card>
            
            <Card>
              <CardHeader>
                <Zap className="mb-2 h-10 w-10 text-primary" />
                <CardTitle>Real-time Data Streams</CardTitle>
                <CardDescription>
                  Access live vehicle telemetry, GPS, diagnostics, and more
                </CardDescription>
              </CardHeader>
            </Card>
            
            <Card>
              <CardHeader>
                <DollarSign className="mb-2 h-10 w-10 text-primary" />
                <CardTitle>Fair Pricing</CardTitle>
                <CardDescription>
                  Vehicle owners set prices, buyers pay directly - no middlemen
                </CardDescription>
              </CardHeader>
            </Card>
            
            <Card>
              <CardHeader>
                <Users className="mb-2 h-10 w-10 text-primary" />
                <CardTitle>Community Driven</CardTitle>
                <CardDescription>
                  Join a growing network of vehicle owners and data consumers
                </CardDescription>
              </CardHeader>
            </Card>
            
            <Card>
              <CardHeader>
                <Car className="mb-2 h-10 w-10 text-primary" />
                <CardTitle>Multi-Vehicle Support</CardTitle>
                <CardDescription>
                  Cars, trucks, motorcycles, fleet vehicles - all supported
                </CardDescription>
              </CardHeader>
            </Card>
            
            <Card>
              <CardHeader>
                <TrendingUp className="mb-2 h-10 w-10 text-primary" />
                <CardTitle>Analytics & Insights</CardTitle>
                <CardDescription>
                  Rich analytics for both data providers and consumers
                </CardDescription>
              </CardHeader>
            </Card>
          </div>
        </div>
      </section>

      {/* How it Works */}
      <section className="container mx-auto px-4 py-16">
        <div className="mx-auto max-w-4xl">
          <h2 className="mb-12 text-center text-3xl font-bold">How It Works</h2>
          <div className="grid gap-8 md:grid-cols-3">
            <div className="text-center">
              <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
                <span className="text-2xl font-bold text-primary">1</span>
              </div>
              <h3 className="mb-2 text-xl font-semibold">Connect Your Vehicle</h3>
              <p className="text-muted-foreground">
                Register your vehicle and configure data streams you want to monetize
              </p>
            </div>
            
            <div className="text-center">
              <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
                <span className="text-2xl font-bold text-primary">2</span>
              </div>
              <h3 className="mb-2 text-xl font-semibold">Set Your Price</h3>
              <p className="text-muted-foreground">
                List your data streams with competitive pricing on the marketplace
              </p>
            </div>
            
            <div className="text-center">
              <div className="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
                <span className="text-2xl font-bold text-primary">3</span>
              </div>
              <h3 className="mb-2 text-xl font-semibold">Earn Crypto</h3>
              <p className="text-muted-foreground">
                Get paid instantly when buyers purchase access to your vehicle data
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="container mx-auto px-4 py-16">
        <Card className="mx-auto max-w-2xl text-center">
          <CardHeader>
            <CardTitle className="text-2xl">Ready to Get Started?</CardTitle>
            <CardDescription>
              Connect your wallet and start exploring the marketplace today
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <ConnectWallet />
            <p className="text-sm text-muted-foreground">
              New to crypto? No problem! We support all major wallet providers.
            </p>
          </CardContent>
        </Card>
      </section>
    </div>
  );
}
