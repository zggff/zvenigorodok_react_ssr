import React from 'react'
import { YMaps, Map, Placemark } from '@pbe/react-yandex-maps'

import Certificate from '../assets/images/certificate_small.webp'
import Reviews from '../components/Reviews'
import { Helmet } from 'react-helmet'

const Home: React.FC = () => {
    const center = [55.746309, 36.878061]

    const defaultState = {
        center: center,
        zoom: 16,
    }
    return (
        <>
            <Helmet>
                <meta
                    name='keywords'
                    content='Звенигород, шиномонтаж, запись, zvenigorodok, звенигородок'
                />
                <meta name='description' content='шиномонтаж в Звенигороде' />
            </Helmet>
            <h1 className='text-center text-red-500 text-4xl'>шиномонтаж</h1>

            <h2 className='text-2xl'>
                Сезонный шиномонтаж в Звенигороде для вашего удобства
            </h2>
            <p>Без очередей и стресса!</p>
            <p>
                Только по записи:
                <br />
                +7(916)-683-46-38
            </p>
            <p>c 8:00 до 22:00 без выходных</p>

            <div className='my-2 md:flex md:gap-2 container'>
                <YMaps>
                    <Map
                        className='w-full h-100 bg-red-100 mb-2 md:w-1/2 md:mb-0'
                        defaultState={defaultState}
                    >
                        <Placemark geometry={center} />
                    </Map>
                </YMaps>
                <img
                    className='w-full  md:h-100 md:w-auto'
                    src={Certificate}
                    alt='Сертификат'
                />
            </div>
            <p>Оборудование: Hofmann</p>
            <p>Расходные материалы: Clipper, Rema Tip-Top</p>
            <Reviews target='Tyres' />
        </>
    )
}

export default Home
