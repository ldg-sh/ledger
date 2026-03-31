"use client";
import {
  createContext,
  useContext,
  useState,
  ReactNode,
  useEffect,
} from "react";

interface LoadingContextType {
  loading: boolean;
  setLoading: (loading: boolean) => void;
}

const LoadingContext = createContext<LoadingContextType | undefined>(undefined);

let globalSetLoading: (loading: boolean) => void = () => {
  console.warn("setLoading called before LoadingProvider was initialized.");
};

export const setGlobalLoading = (loading: boolean) => {
  globalSetLoading(loading);
};

export function LoadingProvider({ children }: { children: ReactNode }) {
  const [loading, setLoading] = useState<boolean>(false);

  useEffect(() => {
    globalSetLoading = setLoading;
  }, []);

  return (
    <LoadingContext.Provider value={{ loading, setLoading }}>
      {children}
    </LoadingContext.Provider>
  );
}

export const useLoading = () => {
  const context = useContext(LoadingContext);
  if (!context)
    throw new Error("useLoading must be used within a LoadingProvider");
  return context;
};
