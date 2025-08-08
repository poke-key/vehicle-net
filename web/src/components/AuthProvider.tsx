'use client';

import React, { createContext, useReducer, useEffect, ReactNode } from 'react';
import { AuthState, AuthAction, User } from '@/types/auth';
import { connectPorto, disconnectPorto, getPortoAccount } from '@/lib/porto';

const initialState: AuthState = {
  user: null,
  isLoading: true,
  error: null,
};

const authReducer = (state: AuthState, action: AuthAction): AuthState => {
  switch (action.type) {
    case 'AUTH_START':
      return {
        ...state,
        isLoading: true,
        error: null,
      };
    case 'AUTH_SUCCESS':
      return {
        ...state,
        user: action.payload,
        isLoading: false,
        error: null,
      };
    case 'AUTH_ERROR':
      return {
        ...state,
        user: null,
        isLoading: false,
        error: action.payload,
      };
    case 'AUTH_LOGOUT':
      return {
        ...state,
        user: null,
        isLoading: false,
        error: null,
      };
    default:
      return state;
  }
};

interface AuthContextType extends AuthState {
  login: () => Promise<void>;
  logout: () => Promise<void>;
}

export const AuthContext = createContext<AuthContextType | null>(null);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [state, dispatch] = useReducer(authReducer, initialState);

  const login = async () => {
    dispatch({ type: 'AUTH_START' });
    try {
      const account = await connectPorto();
      const user: User = {
        address: account,
        isAuthenticated: true,
      };
      dispatch({ type: 'AUTH_SUCCESS', payload: user });
    } catch (error) {
      dispatch({ 
        type: 'AUTH_ERROR', 
        payload: error instanceof Error ? error.message : 'Authentication failed' 
      });
    }
  };

  const logout = async () => {
    dispatch({ type: 'AUTH_START' });
    try {
      await disconnectPorto();
      dispatch({ type: 'AUTH_LOGOUT' });
    } catch (error) {
      console.error('Logout error:', error);
      dispatch({ type: 'AUTH_LOGOUT' });
    }
  };

  useEffect(() => {
    const checkExistingConnection = async () => {
      try {
        const account = await getPortoAccount();
        if (account) {
          const user: User = {
            address: account,
            isAuthenticated: true,
          };
          dispatch({ type: 'AUTH_SUCCESS', payload: user });
        } else {
          dispatch({ type: 'AUTH_LOGOUT' });
        }
      } catch (error) {
        dispatch({ type: 'AUTH_LOGOUT' });
      }
    };

    checkExistingConnection();
  }, []);

  return (
    <AuthContext.Provider value={{
      ...state,
      login,
      logout,
    }}>
      {children}
    </AuthContext.Provider>
  );
};