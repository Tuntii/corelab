/**
 * CoreLab API Bindings
 * 
 * TypeScript wrappers for Tauri IPC commands
 */

import { invoke } from '@tauri-apps/api/core'

// Types matching Rust structs
export interface Person {
    id: number
    name: string
    notes: string | null
    is_active: boolean
    created_at: string
}

export interface Conversation {
    id: number
    person_id: number
    content: string
    context: string | null
    created_at: string
}

export interface Memory {
    id: number
    person_id: number
    key: string
    value: string
    importance: number
    created_at: string
}

// Person API
export const personApi = {
    getAll: () => invoke<Person[]>('get_persons'),

    create: (name: string, notes?: string) =>
        invoke<number>('create_person', { name, notes }),

    update: (id: number, name: string, notes: string | null, isActive: boolean) =>
        invoke<void>('update_person', { id, name, notes, isActive }),
}

// Conversation API
export const conversationApi = {
    getByPerson: (personId: number) =>
        invoke<Conversation[]>('get_conversations', { personId }),

    create: (personId: number, content: string, context?: string) =>
        invoke<number>('create_conversation', { personId, content, context }),
}

// Memory API
export const memoryApi = {
    getByPerson: (personId: number) =>
        invoke<Memory[]>('get_memories', { personId }),

    create: (personId: number, key: string, value: string, importance: number) =>
        invoke<number>('create_memory', { personId, key, value, importance }),
}
