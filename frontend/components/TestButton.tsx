"use client";

import { listFiles } from "@/lib/api/file";

export default function TestButton() {
  return (
    <button
      onClick={() => {
        listFiles("hello2")
          .then((files) => {})
          .catch((err) => {
            console.error("Error listing files:", err);
          });
      }}
    >
      Test Button
    </button>
  );
}
