import { useEffect, useState } from 'react'
import { useLocation } from 'react-router-dom'
import { Link } from 'react-router-dom'

const mdxModules = Object.entries(import.meta.glob('../pages/**/*.mdx')).map(([path, module]) => ({ module: module, path: path.slice(9, -4) }))

function DocumentPage({ children }) {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const [MdxComponent, setMdxComponent] = useState(null)
  const [hasError, setHasError] = useState(false)

  const location = useLocation()

  useEffect(() => {
    setSidebarOpen(false)
    setHasError(false)
    const path = location.pathname.replace(/^\/|\/$/g, '') || 'index'
    mdxModules.find((n) => n.path === path)?.module()
        .then((n) => setMdxComponent(() => n.default))
        .catch(() => setHasError(true))
      ?? setHasError(true)
  }, [location])

  return hasError
    ? (<div>Page not found.</div>)
    : (
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
                <button className='menu-toggle' onClick={() => setSidebarOpen((prev) => !prev)}>
                  ☰
                </button>
                <div className='logo'>Fuzzy, a programming language.</div>
              </div>
            </header>
            <main className='main-content'>
              <div className='mdx-content'>
                {MdxComponent && <MdxComponent />}
              </div>
            </main>
          </div>
        </div>
      </div>
    )
}

export default DocumentPage
