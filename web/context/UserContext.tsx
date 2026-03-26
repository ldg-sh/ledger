"use client";

import { useRouter } from "next/navigation";
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

const REDIRECT_BLACKLIST = ["/login", "/callback/**"];
const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

const UserContext = createContext<UserContextType | undefined>(undefined);

export function UserProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  const router = useRouter();

  const initAuth = async () => {
    try {
      let res = await fetch(`${EDGE_URL}/user/info`, { credentials: "include" });

      if (res.status === 401) {
        const refreshRes = await fetch(`/auth/refresh`, {
          credentials: "include",
          method: "POST",
        });

        if (refreshRes.ok) {
          res = await fetch(`${EDGE_URL}/user/info`, { credentials: "include" });
        }
      }

      if (res.ok) {
        const data = await res.json();
        setUser(data);
      } else {
        attemptRedirect();

        setUser(null);
      }
    } catch (err) {
      setUser(null);

      attemptRedirect();
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    initAuth();

    document.addEventListener("reloadUser", initAuth);

    return () => {
      document.removeEventListener("reloadUser", initAuth);
    };
  }, []);

  function attemptRedirect() {
    const isBlacklisted = REDIRECT_BLACKLIST.some((pattern) => {
      const regexPattern = pattern
        .replace(/\*\*/g, ".*")
        .replace(/\*/g, "[^/]*");
      const regex = new RegExp(`^${regexPattern}$`);
      return regex.test(window.location.pathname);
    });

    if (!isBlacklisted) {
      router.push("/login");
    }
  }

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

export const logout = async () => {
  await fetch(`/auth/logout`, {
    method: "POST",
    credentials: "include",
  });

  document.dispatchEvent(new CustomEvent("reloadUser"));
};