import { authenticatedFetch } from '../../../../lib/api/apiClient';

export async function GET(request: Request, { params }: { params: Promise<{ fileId: string }> }) {
  const { fileId } = await params;
  const { searchParams } = new URL(request.url);
  const preview = searchParams.get("preview") === "true";

  const res = await authenticatedFetch(`/download/${fileId}/view?preview=${preview}`);

  if (!res.ok) return new Response("Error", { status: res.status });

  const headers = new Headers();
  
  headers.set('Content-Type', res.headers.get('Content-Type') || 'application/octet-stream');
  
  const totalSize = res.headers.get('Content-Length');
  if (totalSize) {
    headers.set('Content-Length', totalSize);
  }

  const fileName = res.headers.get('Content-Disposition')?.split('filename=')[1]?.replace(/"/g, '') || 'file';
  headers.set('Content-Disposition', preview ? 'inline' : `attachment; filename="${fileName}"`);

  headers.set('X-Content-Type-Options', 'nosniff');
  headers.set('X-Accel-Buffering', 'no');
  headers.set('Cache-Control', 'no-transform');

  return new Response(res.body, { headers });
}