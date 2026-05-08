import {
  CheckCheck,
  Clipboard,
  ClipboardPaste,
  Download,
  Edit3,
  ExternalLink,
  Link,
  LogOut,
  PencilLine,
  Share2,
  Trash2,
  type LucideIcon,
} from "lucide-react";

export const ICON_REGISTRY: Record<string, LucideIcon> = {
  "trash-2": Trash2,
  edit: Edit3,
  share: Share2,
  copy: Clipboard,
  "external-link": ExternalLink,
  "clipboard-paste": ClipboardPaste,
  "pencil-line": PencilLine,
  link: Link,
  download: Download,
  "check-check": CheckCheck,
  "log-out": LogOut,
};

export type IconName = keyof typeof ICON_REGISTRY;
