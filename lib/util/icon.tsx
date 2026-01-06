export function getFileIcon(fileType: string): string {
    switch (fileType) {
        case 'folder':
            "folder"
            break;
        case 'image/png':
        case 'image/jpeg':
        case 'image/gif':
            return "image";
        case 'application/pdf':
        case 'text/plain':
            return "file-text";
        case 'application/zip':
            return "file-archive";
        case 'video/mp4':
            return "file-video";
        case 'audio/mpeg':
            return "file-audio";
        default:
            return "file";
    }

    return "file";
}