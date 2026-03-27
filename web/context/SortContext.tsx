"use client";
import { createContext, useContext, useState, ReactNode, useEffect } from "react";

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

  useEffect(() => {
    const savedSort = window.localStorage.getItem("sortPreference") as SortOption;
    if (savedSort) {
      setSort(savedSort);
    }
  }, []);

  const toggleSort = (category: string) => {
    setSort((prev) => {
      const isCurrent = prev.startsWith(category);
      const isAsc = prev.endsWith("_asc");

      window.localStorage.setItem("sortPreference", isCurrent && isAsc ? `${category}_desc` : `${category}_asc`);

      return (
        isCurrent && isAsc ? `${category}_desc` : `${category}_asc`
      ) as SortOption;
    });
  };

  return (
    <SortContext.Provider value={{ sort, setSort, toggleSort }}>
      {children}
    </SortContext.Provider>
  );
}

export const useSort = () => {
  const context = useContext(SortContext);
  if (!context) throw new Error("useSort must be used within a SortProvider");
  return context;
};
