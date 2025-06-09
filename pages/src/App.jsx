import { Suspense, useEffect, useState } from 'react'
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom'
import Layout from './components/Layout'

function MdxPage() {
  const location = useLocation()

  const [MdxComponent, setMdxComponent] = useState(null)
  const [hasError, setHasError] = useState(false)

  useEffect(() => {
    setHasError(false)
    const filename = location.pathname.replace(/^\/|\/$/g, '') || 'index'
    import(`./pages/${filename}.mdx`)
      .then((n) => setMdxComponent(() => n.default))
      .catch(() => setHasError(true))
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
