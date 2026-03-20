"use client";

import {
  createContext,
  useContext,
  useEffect,
  useState,
  ReactNode,
} from "react";

interface User {
  username: string;
  email: string;
  avatar_url: string;
}

interface UserContextType {
  user: User | null;
  loading: boolean;
}

const UserContext = createContext<UserContextType | undefined>(undefined);

export function UserProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
useEffect(() => {
  const initAuth = async () => {
    try {
      let res = await fetch("/auth/info", { credentials: "include" });

      if (res.status === 401) {
        const refreshRes = await fetch("/auth/refresh", {
          credentials: "include",
          method: "POST",
        });

        if (refreshRes.ok) {
          res = await fetch("/auth/info", { credentials: "include" });
        }
      }

      if (res.ok) {
        const data = await res.json();
        setUser(data);
      } else {
        setUser(null);
      }
    } catch (err) {
      console.error("Auth initialization failed", err);
      setUser(null);
    } finally {
      setLoading(false);
    }
  };

  initAuth();
}, []);

  return (
    <UserContext.Provider value={{ user, loading }}>
      {children}
    </UserContext.Provider>
  );
}

export const useUser = () => {
  const context = useContext(UserContext);

  if (!context) throw new Error("useUser must be used within a UserProvider");
  return context;
};
