import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { AuthProvider } from './context/AuthContext';
import SignIn from './components/signIn/SignIn';
import EventSelector from './components/events/EventSelector';
import QRCodeReader from './components/codeReader/QrCodeReader';

function App() {
  return (
    <AuthProvider>
      <Router>
        <Routes>
          <Route path='/' element={<SignIn />} />
          <Route path='/events' element={<EventSelector />} />
          <Route path='/codereader' element={<QRCodeReader />} />
        </Routes>
      </Router>
    </AuthProvider>
  );
}

export default App;
