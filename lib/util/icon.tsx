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
            return "pdf";
        case 'text/plain':
            return "file-text";
        default:
            return "file";
    }

    return "file";
}