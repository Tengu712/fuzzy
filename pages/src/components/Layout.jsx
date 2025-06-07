import React, { useState } from 'react'

function Layout({ children }) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  const toggleSidebar = () => {
    setSidebarOpen(!sidebarOpen)
  }

  return (
    <div className="app">
      <header className="header">
        <div className="header-content">
          <button className="menu-toggle" onClick={toggleSidebar}>
            ☰
          </button>
          <div className="logo">Fuzzy</div>
        </div>
      </header>
      <div className="layout">
        <aside className={`sidebar ${sidebarOpen ? 'open' : ''}`}>
          <nav>
            <p>Navigation</p>
          </nav>
        </aside>
        <main className="main-content">
          {children}
        </main>
      </div>
    </div>
  )
}

export default Layout
