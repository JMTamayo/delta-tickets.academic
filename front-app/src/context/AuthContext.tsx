import React, { createContext, useState, useContext, ReactNode } from 'react';

// Define the context type
interface AuthContextType {
    userEmail: string | null;
    login: (email: string) => void;
    logout: () => void;
}

// Create the context
const AuthContext = createContext<AuthContextType | undefined>(undefined);

// Provider for the context
export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
    const [userEmail, setUserEmail] = useState<string | null>(null);

    const login = (email: string) => {
        setUserEmail(email);
    };

    const logout = () => {
        setUserEmail(null);
    };

    return (
        <AuthContext.Provider value={{ userEmail, login, logout }}>
            {children}
        </AuthContext.Provider>
    );
};

// Custom hook to use the Auth context
export const useAuth = (): AuthContextType => {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
};