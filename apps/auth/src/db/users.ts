import { prisma } from "./client";

export const users = {
    findById: (id: string) => {
        return prisma.user.findUnique({ where: { id } });
    },

    findByEmail: (email: string) => {
        return prisma.user.findUnique({ where: { email } });
    },

    setAdmin: (userId: string, isAdmin: boolean) => {
        return prisma.user.update({
            where: { id: userId },
            data: { isAdmin },
        });
    },
};
