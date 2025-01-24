import { createContext, useContext, createSignal, Accessor, Setter } from "solid-js";


interface AuthContextType{
  token: Accessor<String>,
  setToken: Setter<string>,
}

const AuthContext = createContext<AuthContextType>();

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return ctx;
}

export function AuthProvider(props: { children: any }) {
  const [token, setToken] = createSignal('');

  return (
    <AuthContext.Provider value={{ token, setToken }}>
      {props.children}
    </AuthContext.Provider>
  );
}