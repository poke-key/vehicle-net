'use client'

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"

export default function ListVehiclePage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <Card>
        <CardHeader>
          <CardTitle>Vehicle Listing</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground">
            This feature is currently disabled. Use the Rust application to add vehicle nodes directly to the blockchain.
          </p>
        </CardContent>
      </Card>
    </div>
  )
}