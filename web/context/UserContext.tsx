"use client";

import { createStore, del, get, set } from "idb-keyval";
import { useRouter } from "next/navigation";
import {
  createContext,
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
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

const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

const UserContext = createContext<UserContextType | undefined>(undefined);

const ledgerStore = createStore("ledger-user", "user-cache");

export function UserProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const router = useRouter();

  const initAuth = useCallback(async (callback?: () => void) => {
    const sessionExists = document.cookie.includes("session_exists=true");

    const potentialCache = await get<User>("user", ledgerStore);

    if (potentialCache && sessionExists) {
      setUser(potentialCache);
      setLoading(false);
      callback?.();
      return;
    } else if (potentialCache && !sessionExists) {
      await del("user", ledgerStore);
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
        } else {
          setUser(null);
          await del("user", ledgerStore);
          setLoading(false);

          if (sessionExists) {
            router.push("/login");
          }
          return;
        }
      }

      if (res.ok) {
        const data = await res.json();
        setUser(data);

        await set("user", data, ledgerStore);
        callback?.();
      } else {
        setUser(null);
        console.error("Failed to fetch user info, status:", res.status);

        await del("user", ledgerStore);
      }
    } catch (error) {
      console.error("Error during authentication:", error);
      setUser(null);

      await del("user", ledgerStore);

      console.error("Error fetching user info:", error);
    } finally {
      setLoading(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    initAuth();

    const handleReload = (callback?: () => void) => initAuth(callback);
    document.addEventListener("reload-user", (event) => {
      if (event instanceof CustomEvent && typeof event.detail === "function") {
        handleReload(event.detail);
      } else {
        handleReload();
      }
    });

    return () => {
      document.removeEventListener("reload-user", (event) => {
        if (
          event instanceof CustomEvent &&
          typeof event.detail === "function"
        ) {
          handleReload(event.detail);
        } else {
          handleReload();
        }
      });
    };
  }, [initAuth]);

  const value = useMemo(() => ({ user, loading }), [user, loading]);

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
