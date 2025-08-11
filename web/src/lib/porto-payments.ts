import { create } from 'porto/Porto'

export interface DataProduct {
  id: string
  name: string
  description: string
  price: string
  currency: string
  seller: string
}

export class PortoPayments {
  private porto: any

  constructor() {
    this.porto = null
  }

  async initialize() {
    try {
      this.porto = create()
      return true
    } catch (error) {
      console.error('Failed to initialize Porto:', error)
      return false
    }
  }

  async purchaseDataAccess(product: DataProduct) {
    if (!this.porto) {
      throw new Error('Porto not initialized')
    }

    try {
      // Connect wallet if needed
      const accounts = await this.porto.provider.request({
        method: 'wallet_connect'
      })

      if (!accounts?.length) {
        throw new Error('No accounts connected')
      }

      // For now, return a mock transaction
      return {
        success: true,
        transactionHash: '0x' + Math.random().toString(16).substr(2, 64),
        account: accounts[0]
      }
    } catch (error) {
      console.error('Purchase failed:', error)
      throw error
    }
  }

  async getConnectedAccount() {
    if (!this.porto) {
      return null
    }

    try {
      const accounts = await this.porto.provider.request({
        method: 'eth_accounts'
      })
      return accounts[0] || null
    } catch (error) {
      console.error('Failed to get account:', error)
      return null
    }
  }
}

export const portoPayments = new PortoPayments()

export function formatEthValue(valueInWei: string | number): string {
  const wei = typeof valueInWei === 'string' ? BigInt(valueInWei) : BigInt(valueInWei)
  const eth = Number(wei) / 1e18
  return eth.toFixed(4)
}

export function calculatePurchaseCost(pricePerHour: string, durationInSeconds: number): string {
  const pricePerHourWei = BigInt(pricePerHour)
  const hours = Math.ceil(durationInSeconds / 3600)
  const totalCost = pricePerHourWei * BigInt(hours)
  return totalCost.toString()
}

export async function purchaseDataAccessWithPorto(params: {
  productId: string
  durationInSeconds: number
  priceInWei: string
}): Promise<string> {
  const initialized = await portoPayments.initialize()
  if (!initialized) {
    throw new Error('Failed to initialize Porto payments')
  }
  
  const result = await portoPayments.purchaseDataAccess({
    id: params.productId,
    name: `Vehicle Data Access`,
    description: `Access to vehicle data for ${params.durationInSeconds} seconds`,
    price: params.priceInWei,
    currency: 'ETH',
    seller: '0x0000000000000000000000000000000000000000'
  })
  
  return result.transactionHash
}