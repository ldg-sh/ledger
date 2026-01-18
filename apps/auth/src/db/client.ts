import { PrismaClient } from "../generated/prisma";

export const prisma = new PrismaClient();

export type { PrismaClient };
