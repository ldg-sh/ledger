"use client";

import {
  createContext,
  useContext,
  useState,
  ReactNode,
  useEffect,
} from "react";

export type SortOption =
  | "name_desc"
  | "name_asc"
  | "date_desc"
  | "date_asc"
  | "size_desc"
  | "size_asc"
  | "type_desc"
  | "type_asc";

interface SortContextType {
  sort: SortOption;
  setSort: (newSort: SortOption) => void;
  toggleSort: (category: string) => void;
}

const SortContext = createContext<SortContextType | undefined>(undefined);

export function SortProvider({ children }: { children: ReactNode }) {
  const [sort, setSort] = useState<SortOption>("name_asc");
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    const savedSort = window.localStorage.getItem(
      "sortPreference",
    ) as SortOption;
    
    if (savedSort && savedSort !== "name_asc") {
      queueMicrotask(() => {
        setSort(savedSort);
      });
    }

    queueMicrotask(() => {
      setIsMounted(true);
    });
  }, []);

  const toggleSort = (category: string) => {
    setSort((prev) => {
      const isCurrent = prev.startsWith(category);
      const isAsc = prev.endsWith("_asc");
      const nextSort = (
        isCurrent && isAsc ? `${category}_desc` : `${category}_asc`
      ) as SortOption;

      window.localStorage.setItem("sortPreference", nextSort);
      return nextSort;
    });
  };

  return (
    <SortContext.Provider value={{ sort, setSort, toggleSort }}>
      <div style={{ visibility: isMounted ? "visible" : "hidden" }}>
        {children}
      </div>
    </SortContext.Provider>
  );
}

export const useSort = () => {
  const context = useContext(SortContext);
  if (!context) throw new Error("useSort must be used within a SortProvider");
  return context;
};
