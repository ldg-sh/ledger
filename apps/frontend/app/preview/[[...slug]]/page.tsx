import FilePreview from "@/components/preview/FilePreview";

export default async function Page({
  params,
}: {
  params: Promise<{ slug: string }>,
}) {
    const { slug } = await params;

    return (
        <div>
            <FilePreview fileId={(slug as unknown as string[]).join("/")} />
        </div>
    );
}