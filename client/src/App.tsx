import type { ReactElement } from 'react'
import { lazy, Suspense } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import React, { useState, useEffect } from 'react';
import { socket } from './utils';

// const Gallery = lazy(async () => import('pages/Gallery'))
// const Details = lazy(async () => import('pages/Details'))

export default function App(): ReactElement {

	const [isConnected, setIsConnected] = useState(socket.connected);
	const [listenerCount, setListenerCount] = useState(0);
	;
	// get the amount of listeners connected to the server and display it on the screen

	// refresh the listener count every 30 seconds
	// setInterval(() => {
	// 	socket.emit('get_listener_count');
	// }, 30000)


  useEffect(() => {
    function onConnect() {
      setIsConnected(true);
    }

	socket.on('listener_count', (listenerCount: number) => {
		setListenerCount(listenerCount);
		console.log(`Listener Count: ${listenerCount}`);
	})


    function onDisconnect() {
      setIsConnected(false);
    }

    socket.on('connect', onConnect);
    socket.on('disconnect', onDisconnect);

    return () => {
      socket.off('connect', onConnect);
      socket.off('disconnect', onDisconnect);
    };
  }, []);
	return (
		<BrowserRouter>
		<h1>Testing!</h1>
			<h2>Listener Count: {listenerCount}</h2>
			{/* information about connection below */}
			<div>
				<h2>Connection Status: {isConnected ? 'Connected' : 'Disconnected'}</h2>
			</div>
		</BrowserRouter>
	)
}
