import { useState, useEffect } from 'react'
import { personApi, Person } from '@/lib/api'
import PersonList from './PersonList'
import PersonDetail from './PersonDetail'
import './MemoryApp.css'

export default function MemoryApp() {
    const [persons, setPersons] = useState<Person[]>([])
    const [selectedPerson, setSelectedPerson] = useState<Person | null>(null)
    const [isLoading, setIsLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)

    const loadPersons = async () => {
        try {
            setIsLoading(true)
            const data = await personApi.getAll()
            setPersons(data)
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to load persons')
        } finally {
            setIsLoading(false)
        }
    }

    useEffect(() => {
        loadPersons()
    }, [])

    const handlePersonCreated = (id: number, name: string) => {
        const newPerson: Person = {
            id,
            name,
            notes: null,
            is_active: true,
            created_at: new Date().toISOString(),
        }
        setPersons([...persons, newPerson])
        setSelectedPerson(newPerson)
    }

    const handlePersonUpdated = (updatedPerson: Person) => {
        setPersons(persons.map(p => p.id === updatedPerson.id ? updatedPerson : p))
        if (selectedPerson?.id === updatedPerson.id) {
            setSelectedPerson(updatedPerson)
        }
    }

    if (error) {
        return (
            <div className="memory-app error-state">
                <p>❌ {error}</p>
                <button onClick={loadPersons}>Retry</button>
            </div>
        )
    }

    return (
        <div className="memory-app">
            <div className="memory-sidebar">
                <PersonList
                    persons={persons}
                    selectedId={selectedPerson?.id}
                    onSelect={setSelectedPerson}
                    onPersonCreated={handlePersonCreated}
                    isLoading={isLoading}
                />
            </div>

            <div className="memory-main">
                {selectedPerson ? (
                    <PersonDetail
                        person={selectedPerson}
                        onUpdate={handlePersonUpdated}
                    />
                ) : (
                    <div className="memory-empty">
                        <span className="empty-icon">👤</span>
                        <p>Bir kişi seçin veya yeni kişi ekleyin</p>
                    </div>
                )}
            </div>
        </div>
    )
}
