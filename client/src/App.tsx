import React from 'react'
import { Helmet } from 'react-helmet'

import Navbar from './components/Navbar'
import Favicon from './assets/favicon.png'
import Router from './router'

const App: React.FC = () => {
    return (
        <>
            <Helmet>
                <title>звенигородок</title>
                <link rel='icon' href={Favicon} />
            </Helmet>
            <main>
                <Navbar />

                <div className='py-4 px-4'>
                    <Router />
                </div>
            </main>
        </>
    )
}

export default App
