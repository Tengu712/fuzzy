import React, { useState } from 'react'

function Layout({ children, showSidebar = true }) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  const toggleSidebar = () => {
    setSidebarOpen(!sidebarOpen)
  }

  return (
    <div className="app">
      <div className="layout">
        {showSidebar && (
          <aside className={`sidebar ${sidebarOpen ? 'open' : ''}`}>
            <nav>
              <p>Navigation</p>
            </nav>
          </aside>
        )}
        <div className={`main-container ${sidebarOpen && showSidebar ? 'sidebar-open' : ''}`}>
          <header className="header">
            <div className={`header-content ${showSidebar ? 'has-toggle' : ''}`}>
              {showSidebar && (
                <button className="menu-toggle" onClick={toggleSidebar}>
                  ☰
                </button>
              )}
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
