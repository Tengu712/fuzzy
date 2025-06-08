import { useState, Suspense, lazy } from 'react'
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom'
import Layout from './components/Layout'

function MdxPage() {
  const [hasError, setHasError] = useState(false)

  const filename = useLocation().pathname.slice(7) || 'index'
  const LazyMdxComponent = lazy(() => import(`./pages/${filename}.mdx`).catch(() => setHasError(true)))

  if (hasError) return <div>Page not found.</div>

  return (
    <Suspense fallback={<div>Loading...</div>}>
      <Layout>
        <LazyMdxComponent />
      </Layout>
    </Suspense>
  )
}

function App() {
  return (
    <Router>
      <Routes>
        <Route path='*' element={<MdxPage />} />
      </Routes>
    </Router>
  )
}

export default App
