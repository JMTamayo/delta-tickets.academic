import React, { useEffect, useState } from 'react';
import { QRCodeCanvas } from 'qrcode.react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import CssBaseline from '@mui/material/CssBaseline';
import Modal from '@mui/material/Modal';
import CardActionArea from '@mui/material/CardActionArea';
import CardContent from '@mui/material/CardContent';
import CardActions from '@mui/material/CardActions';
import Typography from '@mui/material/Typography';
import Stack from '@mui/material/Stack';
import MuiCard from '@mui/material/Card';
import { styled } from '@mui/material/styles';
import AppTheme from "../../themes/AppTheme";
import ColorModeSelect from "../../themes/ColorModeSelect";
import axios from "axios";
import { useAuth } from "../../context/AuthContext";
import { useNavigate } from 'react-router-dom';

const Card = styled(MuiCard)(({ theme }) => ({
    display: 'flex',
    flexDirection: 'column',
    alignSelf: 'center',
    width: '100%',
    padding: theme.spacing(4),
    gap: theme.spacing(2),
    margin: 'auto',
    [theme.breakpoints.up('sm')]: {
      maxWidth: '450px',
    },
    boxShadow:
      'hsla(220, 30%, 5%, 0.05) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.05) 0px 15px 35px -5px',
    ...theme.applyStyles('dark', {
      boxShadow:
        'hsla(220, 30%, 5%, 0.5) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.08) 0px 15px 35px -5px',
    }),
  }));

const EventsContainer = styled(Stack)(({ theme }) => ({
    padding: 20,
    marginTop: '10vh',
    '&::before': {
      content: '""',
      display: 'block',
      position: 'absolute',
      zIndex: -1,
      inset: 0,
      backgroundImage:
        'radial-gradient(ellipse at 50% 50%, hsl(210, 100%, 97%), hsl(0, 0%, 100%))',
      backgroundRepeat: 'no-repeat',
      ...theme.applyStyles('dark', {
        backgroundImage:
          'radial-gradient(at 50% 50%, hsla(210, 100%, 16%, 0.5), hsl(220, 30%, 5%))',
      }),
    },
  }));

interface Evento {
    id: number;
    nombre: string;
    codigoQR: string;
}

export default function EventSelector(props: { disableCustomTheme?: boolean }) {
    const [eventos, setEventos] = useState<Evento[]>([
        { id: 1, nombre: 'SuperBowl 2025', codigoQR: 'b728d7dc-5d73-4a05-a960-baa3a7705335' },
        { id: 2, nombre: 'Concierto Paul McCartney', codigoQR: 'QR456' },
        { id: 3, nombre: 'Concierto de Feid', codigoQR: 'QR789' },
    ]);
    const [open, setOpen] = useState(false);
    const [eventoSeleccionado, setEventoSeleccionado] = useState<Evento | null>(null);
    const { userEmail, password, logout } = useAuth();
    const navigation = useNavigate();
    const handleClose = () => setOpen(false);

    const handleSelectEvento = (evento: Evento) => {
        setEventoSeleccionado(evento);
        setOpen(true);
    };

    const handleLogout = () => {
        logout();
        navigation('/');
        console.log("User logged out");
        // For example, clear user session or redirect to login page
    };

    return (
        <AppTheme {...props}>
            <CssBaseline enableColorScheme />
            <EventsContainer direction="column" justifyContent="space-between">
                <ColorModeSelect sx={{ position: 'fixed', top: '1rem', right: '1rem' }} />
                <Card variant="outlined">
                    <Typography
                        component="h1"
                        variant="h4"
                        sx={{ width: '100%', fontSize: 'clamp(2rem, 10vw, 2.15rem)' }}
                    >
                        Seleccione un evento
                    </Typography>
                    {eventos.map(evento => (
                        <Card sx={{ maxWidth: 345 }} key={evento.id}>
                        <CardActionArea>
                          <CardContent>
                            <Typography gutterBottom variant="h5" component="div">
                                {evento.nombre}
                            </Typography>
                          </CardContent>
                        </CardActionArea>
                        <CardActions>
                          <Button size="small" color="primary" onClick={() => handleSelectEvento(evento)}>
                            Generar código
                          </Button>
                        </CardActions>
                      </Card>
                    ))}
                    <Button 
                        variant="outlined" 
                        color="secondary" 
                        onClick={handleLogout} 
                        style={{ marginTop: '20px' }}
                    >
                        Cerrar Sesión
                    </Button>
                </Card>
                <Modal
                    open={open}
                    onClose={handleClose}
                    aria-labelledby="modal-modal-title"
                    aria-describedby="modal-modal-description"
                >
                    <Box sx={{ 
                        bgcolor: 'background.paper', 
                        padding: 4, 
                        borderRadius: 2,
                        boxShadow: 24,
                        maxWidth: 400,
                        margin: 'auto',
                        marginTop: '10%',
                    }}>
                        {eventoSeleccionado && (
                            <div>
                                <h2>{eventoSeleccionado.nombre}</h2>
                                <QRCodeCanvas value={eventoSeleccionado.codigoQR} size={256} />
                                <h2>{eventoSeleccionado.codigoQR}</h2>
                            </div>
                        )}
                    </Box>
                </Modal>
            </EventsContainer>
        </AppTheme>
    );
};