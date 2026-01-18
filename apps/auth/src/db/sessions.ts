import { prisma } from "./client";

export const sessions = {
    findByTokenWithUser: (token: string) => {
        return prisma.session.findUnique({
            where: { token },
            include: { user: true },
        });
    },
};
