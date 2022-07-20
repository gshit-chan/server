import { Field, ObjectType, registerEnumType } from 'type-graphql'
import { Thread } from './thread'

@ObjectType()
export class Comment {
	@Field()
	id: string

	@Field()
	message: string

	@Field()
	threadId: string

	@Field(() => Thread)
	thread: Thread

	@Field(() => String, { nullable: true })
	parentId?: string

	@Field(() => Comment, { nullable: true })
	parent?: Comment

	@Field(() => [Comment])
	comments: Comment[]

	@Field()
	createdAt: Date

	@Field()
	updatedAt: Date
}
