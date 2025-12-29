import { useState, useEffect } from 'react'
import { Person, Conversation, Memory, conversationApi, memoryApi, personApi } from '@/lib/api'

interface PersonDetailProps {
    person: Person
    onUpdate: (person: Person) => void
}

export default function PersonDetail({ person, onUpdate }: PersonDetailProps) {
    const [conversations, setConversations] = useState<Conversation[]>([])
    const [memories, setMemories] = useState<Memory[]>([])
    const [newNote, setNewNote] = useState('')
    const [isEditing, setIsEditing] = useState(false)
    const [editName, setEditName] = useState(person.name)
    const [editNotes, setEditNotes] = useState(person.notes || '')

    useEffect(() => {
        loadData()
    }, [person.id])

    useEffect(() => {
        setEditName(person.name)
        setEditNotes(person.notes || '')
    }, [person])

    const loadData = async () => {
        try {
            const [convs, mems] = await Promise.all([
                conversationApi.getByPerson(person.id),
                memoryApi.getByPerson(person.id),
            ])
            setConversations(convs)
            setMemories(mems)
        } catch (err) {
            console.error('Failed to load data:', err)
        }
    }

    const handleSaveNote = async () => {
        if (!newNote.trim()) return

        try {
            const id = await conversationApi.create(person.id, newNote.trim())
            const newConv: Conversation = {
                id,
                person_id: person.id,
                content: newNote.trim(),
                context: null,
                created_at: new Date().toISOString(),
            }
            setConversations([newConv, ...conversations])
            setNewNote('')
        } catch (err) {
            console.error('Failed to save note:', err)
        }
    }

    const handleSaveEdit = async () => {
        try {
            await personApi.update(person.id, editName, editNotes || null, person.is_active)
            onUpdate({
                ...person,
                name: editName,
                notes: editNotes || null,
            })
            setIsEditing(false)
        } catch (err) {
            console.error('Failed to update person:', err)
        }
    }

    const handleDeactivate = async () => {
        try {
            await personApi.update(person.id, person.name, person.notes, false)
            onUpdate({ ...person, is_active: false })
        } catch (err) {
            console.error('Failed to deactivate person:', err)
        }
    }

    return (
        <div className="person-detail">
            <header className="detail-header">
                {isEditing ? (
                    <div className="edit-form">
                        <input
                            type="text"
                            value={editName}
                            onChange={(e) => setEditName(e.target.value)}
                            placeholder="İsim"
                        />
                        <textarea
                            value={editNotes}
                            onChange={(e) => setEditNotes(e.target.value)}
                            placeholder="Notlar..."
                        />
                        <div className="edit-actions">
                            <button onClick={handleSaveEdit} className="btn-primary">Kaydet</button>
                            <button onClick={() => setIsEditing(false)} className="btn-secondary">İptal</button>
                        </div>
                    </div>
                ) : (
                    <>
                        <div className="detail-info">
                            <h1>{person.name}</h1>
                            {person.notes && <p className="notes">{person.notes}</p>}
                        </div>
                        <div className="detail-actions">
                            <button onClick={() => setIsEditing(true)} className="btn-icon">✏️</button>
                            <button onClick={handleDeactivate} className="btn-icon danger">🗑️</button>
                        </div>
                    </>
                )}
            </header>

            <section className="detail-section">
                <h2>Görüşme Notları</h2>
                <div className="note-input">
                    <textarea
                        value={newNote}
                        onChange={(e) => setNewNote(e.target.value)}
                        placeholder="Yeni görüşme notu ekle..."
                    />
                    <button onClick={handleSaveNote} disabled={!newNote.trim()}>
                        Kaydet
                    </button>
                </div>

                <div className="conversation-list">
                    {conversations.length === 0 ? (
                        <p className="empty">Henüz görüşme notu yok</p>
                    ) : (
                        conversations.map((conv) => (
                            <div key={conv.id} className="conversation-item">
                                <p>{conv.content}</p>
                                <span className="date">
                                    {new Date(conv.created_at).toLocaleString('tr-TR')}
                                </span>
                            </div>
                        ))
                    )}
                </div>
            </section>

            <section className="detail-section">
                <h2>Hafızalar</h2>
                <div className="memory-list">
                    {memories.length === 0 ? (
                        <p className="empty">Henüz çıkarılmış hafıza yok</p>
                    ) : (
                        memories.map((mem) => (
                            <div key={mem.id} className="memory-item">
                                <div className="memory-header">
                                    <span className="memory-key">{mem.key}</span>
                                    <span className={`importance importance-${mem.importance}`}>
                                        {'★'.repeat(mem.importance)}
                                    </span>
                                </div>
                                <p>{mem.value}</p>
                            </div>
                        ))
                    )}
                </div>
            </section>
        </div>
    )
}
