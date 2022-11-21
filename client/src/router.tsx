import React from 'react'
import { Route, Routes } from 'react-router-dom'

import NotFound from './pages/404'
import Cleaning from './pages/Cleaning'
import Home from './pages/Home'

const Router: React.FC = () => {
    return (
        <Routes>
            <Route path='/' element={<Home />} />
            <Route path='/cleaning' element={<Cleaning />} />
            <Route path='*' element={<NotFound />} />
        </Routes>
    )
}

export default Router
