import { Suspense, useEffect, useState } from 'react'
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom'
import Layout from './components/Layout'

const mdxModules = Object.entries(import.meta.glob('./pages/**/*.mdx')).map(([path, module]) => ({ module: module, path: path.slice(8, -4) }))

function MdxPage() {
  const location = useLocation()

  const [MdxComponent, setMdxComponent] = useState(null)
  const [hasError, setHasError] = useState(false)

  useEffect(() => {
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
      <Suspense fallback={<div>Loading...</div>}>
        <Layout>
          {MdxComponent && <MdxComponent />}
        </Layout>
      </Suspense>
    )
}

function App() {
  return (
    <Router basename='/fuzzy/'>
      <Routes>
        <Route path='*' element={<MdxPage />} />
      </Routes>
    </Router>
  )
}

export default App
