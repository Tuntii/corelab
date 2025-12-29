import { useState } from 'react'
import { Person, personApi } from '@/lib/api'

interface PersonListProps {
    persons: Person[]
    selectedId?: number
    onSelect: (person: Person) => void
    onPersonCreated: (id: number, name: string) => void
    isLoading: boolean
}

export default function PersonList({
    persons,
    selectedId,
    onSelect,
    onPersonCreated,
    isLoading,
}: PersonListProps) {
    const [newName, setNewName] = useState('')
    const [isCreating, setIsCreating] = useState(false)

    const handleCreate = async () => {
        if (!newName.trim()) return

        try {
            setIsCreating(true)
            const id = await personApi.create(newName.trim())
            onPersonCreated(id, newName.trim())
            setNewName('')
        } catch (err) {
            console.error('Failed to create person:', err)
        } finally {
            setIsCreating(false)
        }
    }

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            handleCreate()
        }
    }

    return (
        <div className="person-list">
            <div className="person-list-header">
                <h2>Kişiler</h2>
                <span className="count">{persons.length}</span>
            </div>

            <div className="person-add">
                <input
                    type="text"
                    placeholder="Yeni kişi ekle..."
                    value={newName}
                    onChange={(e) => setNewName(e.target.value)}
                    onKeyDown={handleKeyDown}
                    disabled={isCreating}
                />
                <button
                    onClick={handleCreate}
                    disabled={!newName.trim() || isCreating}
                    className="add-btn"
                >
                    +
                </button>
            </div>

            <div className="person-list-items">
                {isLoading ? (
                    <div className="loading">Yükleniyor...</div>
                ) : persons.length === 0 ? (
                    <div className="empty">Henüz kişi yok</div>
                ) : (
                    persons.map((person) => (
                        <button
                            key={person.id}
                            className={`person-item ${selectedId === person.id ? 'active' : ''}`}
                            onClick={() => onSelect(person)}
                        >
                            <span className="person-avatar">
                                {person.name.charAt(0).toUpperCase()}
                            </span>
                            <div className="person-info">
                                <span className="person-name">{person.name}</span>
                                <span className="person-date">
                                    {new Date(person.created_at).toLocaleDateString('tr-TR')}
                                </span>
                            </div>
                        </button>
                    ))
                )}
            </div>
        </div>
    )
}
