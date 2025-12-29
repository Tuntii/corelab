import { useState } from 'react'
import Shell from './components/Shell'
import { MemoryApp } from '@apps/memory'

function App() {
    const [activeApp, setActiveApp] = useState('memory')

    return (
        <Shell activeApp={activeApp} onAppChange={setActiveApp}>
            {activeApp === 'memory' && <MemoryApp />}
        </Shell>
    )
}

export default App
