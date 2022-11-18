import React from 'react'
import { Route, Routes } from 'react-router-dom'

import NotFound from './pages/404'
import Home from './pages/Home'

const Router: React.FC = () => {
    return (
        <Routes>
            <Route path='/' element={<Home />} />
            <Route path='*' element={<NotFound />} />
        </Routes>
    )
}

export default Router
