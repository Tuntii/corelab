import React from 'react'
import TabNav from './TabNav'
import './Shell.css'

interface ShellProps {
    activeApp: string
    onAppChange: (app: string) => void
    children: React.ReactNode
}

const apps = [
    { id: 'memory', name: 'Memory', icon: '🧠' },
    // Future apps will be added here
]

export default function Shell({ activeApp, onAppChange, children }: ShellProps) {
    return (
        <div className="shell">
            <aside className="shell-sidebar">
                <div className="shell-logo">
                    <span className="logo-icon">⚛️</span>
                    <span className="logo-text">CoreLab</span>
                </div>

                <TabNav
                    apps={apps}
                    activeApp={activeApp}
                    onAppChange={onAppChange}
                />

                <div className="shell-footer">
                    <span className="version">v0.1.0</span>
                </div>
            </aside>

            <main className="shell-main">
                {children}
            </main>
        </div>
    )
}
