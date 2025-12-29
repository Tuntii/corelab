interface AppItem {
    id: string
    name: string
    icon: string
}

interface TabNavProps {
    apps: AppItem[]
    activeApp: string
    onAppChange: (app: string) => void
}

export default function TabNav({ apps, activeApp, onAppChange }: TabNavProps) {
    return (
        <nav className="tab-nav">
            {apps.map((app) => (
                <button
                    key={app.id}
                    className={`tab-nav-item ${activeApp === app.id ? 'active' : ''}`}
                    onClick={() => onAppChange(app.id)}
                >
                    <span className="tab-icon">{app.icon}</span>
                    <span className="tab-label">{app.name}</span>
                </button>
            ))}
        </nav>
    )
}
