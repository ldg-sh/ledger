"use client";

import { useSyncExternalStore } from "react";

interface LocalDateProps {
  timestamp: string;
  type?: "full" | "date" | "time";
}

const subscribe = () => () => {};

export default function LocalDate({ timestamp, type = "full" }: LocalDateProps) {
  const isServer = useSyncExternalStore(
    subscribe,
    () => false,
    () => true
  );

  if (isServer || !timestamp) {
    return <span aria-hidden="true">&nbsp;</span>;
  }

  const utcString = timestamp.endsWith("Z") ? timestamp : `${timestamp}Z`;
  const dateObj = new Date(utcString);

  if (isNaN(dateObj.getTime())) {
    return <span>Unknown</span>;
  }

  if (type === "date") {
    return (
      <span>
        {dateObj.toLocaleDateString(undefined, {
          year: "numeric",
          month: "long",
          day: "numeric",
        })}
      </span>
    );
  }

  if (type === "time") {
    return (
      <span>
        {dateObj.toLocaleTimeString(undefined, {
          hour: "2-digit",
          minute: "2-digit",
        })}
      </span>
    );
  }

  return (
    <span>
      {dateObj.toLocaleString(undefined, {
        year: "numeric",
        month: "long",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      })}
    </span>
  );
}
