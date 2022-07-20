import { PrismaClient } from '@prisma/client'
import { Arg, Mutation, Query, Resolver } from 'type-graphql'
import { Chats, Thread } from '../models/thread'

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
		@Arg('title') title: string,
		@Arg('message') message: string,
		@Arg('chat') chat: Chats,
		@Arg('username', { nullable: true }) username: string = 'anon'
	) {
		return await this.prisma.thread.create({
			data: {
				title,
				chat,
				username,
				message,
			},
		})
	}
}
