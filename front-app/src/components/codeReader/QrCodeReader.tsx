import React, { useEffect, useState } from 'react';
import jsQR from 'jsqr';
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

const ReaderContainer = styled(Stack)(({ theme }) => ({
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

export default function QRCodeReader(props: { disableCustomTheme?: boolean }) {
    const [validationMessage, setValidationMessage] = useState('');
    const { userEmail, password, logout } = useAuth();
    const navigation = useNavigate();

    const handleImageChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
        const target = e.target as HTMLInputElement;
        const files = target.files;
        if (files) {
            const file = files[0];
            const reader = new FileReader();
            reader.onloadend = () => {
                const imgElement = document.createElement('img');
                imgElement.src = reader.result as string;
                imgElement.onload = async () => {
                    const canvas = document.createElement('canvas');
                    const context = canvas.getContext('2d');
                    if (context) {
                        canvas.width = imgElement.width;
                        canvas.height = imgElement.height;
                        context.drawImage(imgElement, 0, 0);
                        const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
                        const code = jsQR(imageData.data, canvas.width, canvas.height);
                        console.log(code)
                        if (code) {
                            await validateQRCode(code.data)
                        } else {
                            setValidationMessage('No se encontró código');
                        }
                    }
                };
            };
            reader.readAsDataURL(file);
        } else {
            setValidationMessage('No se seleccionó el archivo');
        }
    };

    const validateQRCode = async (qrCode: string) => {
        try {
            const response = await fetch(`http://localhost:8001/verify-ticket/${qrCode}`, {
                method: 'GET',
                headers: {
                    'username': userEmail as string || '',
                    'key': password as string || ''
                },
            });

            if (!response.ok) {
                throw new Error('Network response was not ok');
            }

            const data = await response.json();
            console.log(data)
            if (response.status == 200) { // Assuming your API returns an `exists` field
                setValidationMessage('Acceso permitido');
            } else {
                setValidationMessage('El código no es válido');
            }
        } catch (error) {
            console.error('Error validating QR Code:', error);
            setValidationMessage('El código no es válido');
        }
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
            <ReaderContainer direction="column" justifyContent="space-between">
                <ColorModeSelect sx={{ position: 'fixed', top: '1rem', right: '1rem' }} />
                <Card variant="outlined">
                    <Typography
                        component="h1"
                        variant="h4"
                        sx={{ width: '100%', fontSize: 'clamp(2rem, 10vw, 2.15rem)' }}
                    >
                        Lector de código
                    </Typography>
                    <label htmlFor="upload-file">
                        <input
                            style={{ display: 'none' }}
                            accept='image/*'
                            id="upload-file"
                            type="file"
                            onChange={handleImageChange}
                        />
                        <Button variant="contained" component="span">
                            Cargar archivo
                        </Button>
                    </label>
                    <Typography
                        component="h2"
                        variant="h6"
                        sx={{ width: '100%' }}
                    >
                        Validación: {validationMessage}
                    </Typography>
                    <Button 
                        variant="outlined" 
                        color="secondary" 
                        onClick={handleLogout} 
                        style={{ marginTop: '20px' }}
                    >
                        Cerrar Sesión
                    </Button>
                </Card>
            </ReaderContainer>
        </AppTheme>
    );
};
