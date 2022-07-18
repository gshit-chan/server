import { PrismaClient } from '@prisma/client'
import { Arg, Mutation, Query, Resolver } from 'type-graphql'
import { Thread } from '../models/thread'

@Resolver(Thread)
export class ThreadResolver {
	private prisma = new PrismaClient()

	@Query(() => Thread, { nullable: true })
	async get_thread(@Arg('id') id: string) {
		return await this.prisma.thread.findUnique({
			where: {
				id,
			},
			include: {
				parent: true,
				comments: true,
			},
		})
	}

	@Query(() => [Thread])
	async get_threads() {
		return await this.prisma.thread.findMany({
			orderBy: {
				createdAt: 'desc',
			},
		})
	}

	@Mutation(() => Thread)
	async create_thread(
		@Arg('message') message: string,
		@Arg('username', { nullable: true }) username: string = 'anon',
		@Arg('userHash', { nullable: true }) userHash?: string,
		@Arg('parentId', { nullable: true }) parentId?: string
	) {
		return await this.prisma.thread.create({
			data: {
				username,
				userHash: userHash ?? '',
				message,
				parentId,
			},
		})
	}
}
