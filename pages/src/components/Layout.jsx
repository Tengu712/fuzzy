import { useState } from 'react'

function Layout({ children }) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  const toggleSidebar = () => setSidebarOpen((prev) => !prev)

  return (
    <div className="app">
      <div className="layout">
        <aside className={`sidebar ${sidebarOpen ? 'open' : ''}`}>
          <nav>
            <p>Navigation</p>
          </nav>
        </aside>
        <div className={`main-container ${sidebarOpen ? 'sidebar-open' : ''}`}>
          <header className="header">
            <div className="header-content">
              <button className="menu-toggle" onClick={toggleSidebar}>
                ☰
              </button>
              <div className="logo">Fuzzy, a programming language.</div>
            </div>
          </header>
          <main className="main-content">
            <div className="mdx-content">
              {children}
            </div>
          </main>
        </div>
      </div>
    </div>
  )
}

export default Layout
