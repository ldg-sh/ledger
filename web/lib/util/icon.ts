import { 
  Trash2, 
  Edit3, 
  Share2, 
  ExternalLink,
  type LucideIcon, 
  ClipboardPaste,
  PencilLine,
  Link,
  Download,
  CheckCheck,
  LogOut,
  Clipboard
} from "lucide-react";

export const ICON_REGISTRY: Record<string, LucideIcon> = {
  "trash-2": Trash2,
  "edit": Edit3,
  "share": Share2,
  "copy": Clipboard,
  "external-link": ExternalLink,
  "clipboard-paste": ClipboardPaste,
  "pencil-line": PencilLine,
  "link": Link,
  "download": Download,
  "check-check": CheckCheck,
  "log-out": LogOut
};

export type IconName = keyof typeof ICON_REGISTRY;