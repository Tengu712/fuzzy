import { Suspense, useEffect, useState } from 'react'
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom'
import DocumentPage from './components/DocumentPage'

function App() {
  return (
    <Router basename='/fuzzy/'>
      <Routes>
        <Route path='*' element={<DocumentPage />} />
      </Routes>
    </Router>
  )
}

export default App
