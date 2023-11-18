export interface HuskeyDatabase {
    entries: PasswordEntry[]
  }
  
export interface PasswordEntry {
    name: string
    username: string
    password: string
    url?: string
}

