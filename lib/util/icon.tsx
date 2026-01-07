export function getFileIcon(fileType: string): string {
    if (fileType === 'folder') {
        return "folder";
    } else if (fileType.startsWith('image/')) {
        return "image";
    } else if (fileType === 'application/pdf' || fileType === 'text/plain') {
        return "file-text";
    } else if (fileType === 'application/zip') {
        return "file-archive";
    } else if (fileType === 'video/mp4') {
        return "file-video";
    } else if (fileType.startsWith('audio/')) {
        return "file-audio";
    }
    
    return "file";
}