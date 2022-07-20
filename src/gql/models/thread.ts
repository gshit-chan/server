import { Field, ObjectType, registerEnumType } from 'type-graphql'

export enum Chats {
	GLOBAL = 'GLOBAL',
}

registerEnumType(Chats, {
	name: 'Chats',
})

@ObjectType()
export class Thread {
	@Field()
	id: string

	@Field()
	title: string

	@Field()
	message: string

	@Field(() => Chats)
	chat: Chats

	@Field(() => String, { nullable: true })
	username?: string

	@Field(() => [Thread])
	comments: Thread[]

	@Field()
	createdAt: Date

	@Field()
	updatedAt: Date
}
