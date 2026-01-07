"use client";

import { downloadFull } from '@/lib/api/file';
import { useState } from 'react';

export default function FilePreview({ fileId, fileType }: { fileId: string, fileType?: string }) {
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);

  const handlePreview = async () => {
    const response = await downloadFull(fileId);

    const blob = new Blob([response], { type: fileType || "application/octet-stream" });
    const url = URL.createObjectURL(blob);
    setPreviewUrl(url);
  };

  return (
    <div>
      <button onClick={handlePreview}>Preview File</button>
      
      {previewUrl && (
        <div className="preview-container">
          <iframe 
            src={previewUrl} 
            width="100%" 
            height="600px" 
            title="File Preview"
          />
        </div>
      )}
    </div>
  );
}