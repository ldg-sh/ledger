"use client";

import { listFiles } from "@/lib/api/file";
import Row from "./Row";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";
import { extractPathFromUrl } from "@/lib/util/url";

export default function FileList() {
  let pathname = usePathname();
  const [data, setData] = useState<{ folders: Folder[]; files: File[] } | null>(
    null
  );

  let [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set());
  let [lastDeliberateClick, setLastDeliberateClick] = useState<string | null>(
    null
  );

  const handleSelectAll = () => {
    if (data) {
      const allFileIds: string[] = [];
      data.folders.forEach((folder) => {
        allFileIds.push(folder.folderName);
      });
      data.files.forEach((file) => {
        allFileIds.push(file.fileId);
      });
      setSelectedFiles(new Set(allFileIds));
    }
  };

  const escape = () => {
    setSelectedFiles(new Set());
    setLastDeliberateClick(null);
  };

  useEffect(() => {
    document.addEventListener("keydown", (event) => {
      if ((event.metaKey || event.ctrlKey) && event.key === "a") {
        event.preventDefault();
        handleSelectAll();
      } else if (event.key === "Escape") {
        event.preventDefault();
        escape();
      }
    });
  });

  function handleRowClick(
    fileId: string,
    selected: boolean,
    isShiftKey: boolean,
    isCommandKey: boolean
  ) {
    console.log("All files selected before:", Array.from(selectedFiles));
    setSelectedFiles((prevSelected) => {
      const newSelected = new Set(prevSelected);

      if (isCommandKey) {
        if (selected) {
          newSelected.delete(fileId);
        } else {
          newSelected.add(fileId);
        }
      } else if (isShiftKey && lastDeliberateClick) {
        newSelected.clear();

        let allFileIds: string[] = [];

        data?.folders.forEach((folder) => {
          allFileIds.push(folder.folderName);
        });
        data?.files.forEach((file) => {
          allFileIds.push(file.fileId);
        });

        let startIndex = allFileIds.indexOf(lastDeliberateClick);
        let endIndex = allFileIds.indexOf(fileId);

        if (startIndex > -1 && endIndex > -1) {
          let [from, to] =
            startIndex < endIndex
              ? [startIndex, endIndex]
              : [endIndex, startIndex];

          for (let i = from; i <= to; i++) {
            newSelected.add(allFileIds[i]);
          }
        }
      } else {
        newSelected.clear();

        newSelected.add(fileId);
        setLastDeliberateClick(fileId);
      }

      return newSelected;
    });
  }

  useEffect(() => {
    let promise = listFiles(extractPathFromUrl(pathname));
    promise.then((res) => {
      setData(res);
    });
  }, [pathname]);

  return (
    <>
      {data?.folders.map((folder) => (
        <Row
          key={folder.folderName}
          fileName={folder.folderName}
          fileSize={folder.size}
          fileType="folder"
          folder={true}
          clickCallback={handleRowClick}
          selected={selectedFiles.has(folder.folderName)}
        />
      ))}
      {data?.files.map((file) => (
        <Row
          key={file.fileId}
          fileName={file.fileName}
          fileSize={file.fileSize}
          fileType={file.fileType}
          fileId={file.fileId}
          createdAt={file.createdAt}
          clickCallback={handleRowClick}
          selected={selectedFiles.has(file.fileId)}
        />
      ))}
    </>
  );
}
