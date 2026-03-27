import { 
  Folder, 
  Image, 
  FileText, 
  FileArchive, 
  FileVideo, 
  FileAudio, 
  HardDrive, 
  File,
  LucideIcon 
} from "lucide-react";

export function getFileIcon(fileType: string): LucideIcon {
    if (fileType === 'folder') {
        return Folder;
    } else if (fileType.startsWith('image/')) {
        return Image;
    } else if (fileType === 'application/pdf' || fileType === 'text/plain') {
        return FileText;
    } else if (fileType === 'application/zip') {
        return FileArchive;
    } else if (fileType.startsWith('video/')) {
        return FileVideo;
    } else if (fileType.startsWith('audio/')) {
        return FileAudio;
    } else if (fileType === "application/x-apple-diskimage") {
        return HardDrive;
    }
    
    return File;
}