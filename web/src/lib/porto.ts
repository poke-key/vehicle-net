import { create } from 'porto/Porto'

let portoInstance: any = null

export function getPorto() {
  if (!portoInstance) {
    portoInstance = create()
  }
  return portoInstance
}

export async function connectPorto() {
  const porto = getPorto()
  
  try {
    const accounts = await porto.provider.request({
      method: 'wallet_connect'
    })
    return accounts
  } catch (error) {
    console.error('Porto connection failed:', error)
    throw error
  }
}

export async function disconnectPorto() {
  const porto = getPorto()
  
  try {
    await porto.provider.request({
      method: 'wallet_disconnect'
    })
  } catch (error) {
    console.error('Porto disconnection failed:', error)
    throw error
  }
}

export async function getPortoAccount() {
  const porto = getPorto()
  
  try {
    const accounts = await porto.provider.request({
      method: 'eth_accounts'
    })
    return accounts[0] || null
  } catch (error) {
    console.error('Failed to get Porto account:', error)
    return null
  }
}