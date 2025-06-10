import { useState } from 'react'
import { Link } from 'react-router-dom'

function Layout({ children }) {
  const [sidebarOpen, setSidebarOpen] = useState(false)

  const toggleSidebar = () => setSidebarOpen((prev) => !prev)

  return (
    <div className='app'>
      <div className='layout'>
        <aside className={`sidebar ${sidebarOpen ? 'open' : ''}`}>
          <nav>
            <Link to='/'                     className='sidebar-link'>Introduction</Link>
            <Link to='/literal-and-keyword/' className='sidebar-link'>Literal & Keyword</Link>
            <Link to='/grammar/'             className='sidebar-link'>Grammar</Link>
            <div>
              <label>Built-in Types</label>
            </div>
            <Link to='/builtin-types/bool/'     className='sidebar-link nested'>bool</Link>
            <Link to='/builtin-types/numeric/'  className='sidebar-link nested'>numeric</Link>
            <Link to='/builtin-types/string/'   className='sidebar-link nested'>string</Link>
            <Link to='/builtin-types/symbol/'   className='sidebar-link nested'>symbol</Link>
            <Link to='/builtin-types/array/'    className='sidebar-link nested'>[]</Link>
            <Link to='/builtin-types/lazy/'     className='sidebar-link nested'>{'{}'}</Link>
            <Link to='/builtin-types/function/' className='sidebar-link nested'>function</Link>
            <Link to='/release-note/' className='sidebar-link'>Release Note</Link>
            <div>
              <a href='https://github.com/Tengu712/fuzzy'>
                <img src='/fuzzy/github-mark-white.svg' alt='GitHub' />
              </a>
            </div>
          </nav>
        </aside>
        <div className={`main-container ${sidebarOpen ? 'sidebar-open' : ''}`}>
          <header className='header'>
            <div className='header-content'>
              <button className='menu-toggle' onClick={toggleSidebar}>
                ☰
              </button>
              <div className='logo'>Fuzzy, a programming language.</div>
            </div>
          </header>
          <main className='main-content'>
            <div className='mdx-content'>
              {children}
            </div>
          </main>
        </div>
      </div>
    </div>
  )
}

export default Layout
