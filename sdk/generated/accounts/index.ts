export * from './StakeEntry'
export * from './StakePool'
export * from './Treasury'

import { Treasury } from './Treasury'
import { StakeEntry } from './StakeEntry'
import { StakePool } from './StakePool'

export const accountProviders = { Treasury, StakeEntry, StakePool }
