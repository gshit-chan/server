import { Field, ObjectType } from 'type-graphql'

@ObjectType()
export class Thread {
	@Field()
	id: string

	@Field()
	userHash: string

	@Field(() => String, { nullable: true })
	username?: string

	@Field()
	message: string

	@Field(() => String, { nullable: true })
	parentId?: string

	@Field(() => Thread, { nullable: true })
	parent?: Thread

	@Field(() => [Thread])
	comments: Thread[]

	@Field()
	createdAt: Date

	@Field()
	updatedAt: Date
}
