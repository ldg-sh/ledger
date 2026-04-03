export default function formatDuration(millis: number): string {
  const seconds = (millis / 1000) % 60;
  const minutes = Math.floor(millis / (1000 * 60)) % 60;
  const hours = Math.floor(millis / (1000 * 60 * 60));
  
  const parts = [];
  if (hours > 0) parts.push(`${hours.toFixed(0)}h`);
  if (minutes > 0) parts.push(`${minutes.toFixed(0)}m`);
  if (seconds > 0 || parts.length === 0) parts.push(`${seconds.toFixed(2)}s`);

  return parts.join(' ');
}