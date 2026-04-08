"use client";

import { createStore, del, get, set } from "idb-keyval";
import { useRouter } from "next/navigation";
import {
  createContext,
  useEffect,
  useState,
  ReactNode,
  useCallback,
  useMemo,
  useContext,
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

const REDIRECT_BLACKLIST = ["/login", "/signup", "/callback/**", "/about"];
const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

const UserContext = createContext<UserContextType | undefined>(undefined);

const ledgerStore = createStore("ledger-user", "user-cache");

export function UserProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  const initAuth = useCallback(async () => {
    const potentialCache = await get<User>("user", ledgerStore);
    if (potentialCache) {
      setUser(potentialCache);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      let res = await fetch(`${EDGE_URL}/user/info`, {
        credentials: "include",
      });

      if (res.status === 401) {
        const refreshRes = await fetch(`/auth/refresh`, {
          method: "POST",
          credentials: "include",
        });
        if (refreshRes.ok) {
          res = await fetch(`${EDGE_URL}/user/info`, {
            credentials: "include",
          });
        }
      }

      if (res.ok) {
        const data = await res.json();
        setUser(data);

        await set("user", data, ledgerStore);
      } else {
        setUser(null);
        console.error("Failed to fetch user info, status:", res.status);
        attemptRedirect();

        await del("user", ledgerStore);
      }
    } catch (error) {
      console.error("Error during authentication:", error);
      setUser(null);
      attemptRedirect();

      await del("user", ledgerStore);

      console.error("Error fetching user info:", error);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    initAuth();

    const handleReload = () => initAuth();
    document.addEventListener("reload-user", handleReload);

    return () => {
      document.removeEventListener("reload-user", handleReload);
    };
  }, [initAuth]);

  const value = useMemo(() => ({ user, loading }), [user, loading]);

  function attemptRedirect() {
    const isBlacklisted = REDIRECT_BLACKLIST.some((pattern) => {
      const regex = new RegExp(
        `^${pattern.replace(/\*\*/g, ".*").replace(/\*/g, "[^/]*")}$`,
      );
      return regex.test(window.location.pathname);
    });

    if (!isBlacklisted) {
      router.push("/login");
    }
  }

  return <UserContext.Provider value={value}>{children}</UserContext.Provider>;
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

  await del("user", ledgerStore);

  document.dispatchEvent(new CustomEvent("reload-user"));
};
