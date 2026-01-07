"use client";

import { listFiles } from "@/lib/api/file";
import Row from "./Row";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";
import { extractPathFromUrl } from "@/lib/util/url";

export default function FileList() {
    let pathname = usePathname();
    const [data, setData] = useState<{ folders: Folder[]; files: File[] } | null>(null);

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
        />
      ))}
      {data?.files.map((file) => (
        <Row
          key={file.fileName}
          fileName={file.fileName}
          fileSize={file.fileSize}
          fileType={file.fileType}
          createdAt={file.createdAt}
        />
      ))}
    </>
  );
}